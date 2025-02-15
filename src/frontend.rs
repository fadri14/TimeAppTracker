use crate::backend;
use rusqlite::{params, Connection, Result};
use chrono::{Duration, NaiveDate, Datelike};

const NUMBER_MINUTES_IN_HOUR: u16 = 60;

enum Type {
    Main,
    App
}

struct Time {
    hour: u16,
    min: u16,
}

impl Time {
    fn new(mins: u16) -> Time {
        if mins <= 0 {
            return Time { hour : 0, min : 0 };
        }
        return Time { hour : mins / NUMBER_MINUTES_IN_HOUR, min : mins % NUMBER_MINUTES_IN_HOUR };
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.hour == 0 {
            return write!(f, "{}m", self.min);
        }
        if self.min < 10 {
            return write!(f, "{}h0{}", self.hour, self.min);
        }
        return write!(f, "{}h{}", self.hour, self.min);
    }
}

struct TimeApp {
    type_data: Type,
    name: String,
    time: Time,
    date: NaiveDate,
    min_total: u16,
}

impl TimeApp {
    fn new(type_data: Type, name: String, date: NaiveDate, mins: u16) -> TimeApp {
        let name = name[1..name.len()-1].to_string();
        TimeApp { type_data, name, time : Time::new(mins), date, min_total : mins}
    }
}

impl std::fmt::Display for TimeApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_data {
            Type::App => {
                return write!(f, "{} : {}", self.name, self.time);
            }
            Type::Main => {
                return write!(f, "{} {} : {}", self.date.weekday(), self.date, self.time);
            }
        }
    }
}

struct Stat {
    max: Time,
    min: Time,
    mean: Time,
}

impl Stat {
    fn new(values: Vec<TimeApp>) -> Stat {
        if values.len() == 0 {
            return Stat { max : Time::new(0), min : Time::new(0), mean : Time::new(0) };
        }

        let mut count = values[0].min_total;
        let mut min = values[0].min_total;
        let mut max = values[0].min_total;

        for i in 1..values.len() {
            count += values[i].min_total;

            if min > values[i].min_total {
                min = values[i].min_total;
            }

            if max < values[i].min_total {
                max = values[i].min_total;
            }
        }

        return Stat { max : Time::new(max), min : Time::new(min), mean : Time::new(count / (values.len() + 1) as u16) };
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Max : {}\nMin : {}\nMean: {}", self.max, self.min, self.mean);
    }
}

pub fn print_main(date: NaiveDate, number_days: u16) -> Result<()> {
    let conn = backend::connect_database()?;

    let values = get_time_main(&conn, date, number_days)?;

    println!("\tPC time : ");
    for v in &values {
        println!("{}", v);
    }

    println!("\n\tStats of PC time :\n{}", Stat::new(values));

    Ok(())
}

pub fn print_apps(date: NaiveDate) -> Result<()> {
    let conn = backend::connect_database()?;

    let values = get_time_apps(&conn, date)?;
    if values.len() > 0 {
        println!("\tApplication time for {} : ", date);
        for v in values {
            println!("{}", v);
        }
    }

    Ok(())
}

fn get_time_main(conn: &Connection, date: NaiveDate, number_days: u16) -> Result<Vec<TimeApp>> {
    let query = format!("SELECT date, main FROM time WHERE date <= ?1 and date >= DATE(?1, '-{} days') ORDER BY date DESC", number_days);
    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map(params![date.to_string()], |row| {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d").expect("Unable to retrieve a date");
        Ok(TimeApp::new(Type::Main, "main".to_string(), date, row.get::<_, u16>(1)?))
    })?;

    let mut values: Vec<TimeApp> = Vec::new();
    for row in rows {
        if let Ok(time) = row {
            values.push(time);
        }
    }

    for i in 0..number_days {
        let deadline = date - Duration::days(i as i64);
        if i == values.len() as u16 || values[i as usize].date != deadline {
            values.insert(i as usize, TimeApp::new(Type::Main, "main".to_string(), deadline, 0));
        }
    }

    Ok(values)
}

fn get_time_apps(conn: &Connection, date: NaiveDate) -> Result<Vec<TimeApp>> {
    let column_names = backend::get_column_name(&conn)?;

    let mut stmt = conn.prepare("SELECT * FROM time WHERE date = ?1")?;
    let mut rows = stmt.query_map(params![date.to_string()], |row| {
        let mut values: Vec<TimeApp> = Vec::new();
        for i in 1..column_names.len() {
            values.push(TimeApp::new(Type::App, column_names[i].clone(), date, row.get::<_, u16>(i+1)?))
        }
        Ok(values)
    })?;

    let mut values: Vec<TimeApp> = Vec::new();
    match rows.next() {
        Some(Ok(vec_times)) => {
            for t in vec_times {
                values.push(t);
            }
        }
        _ => {
            for n in 1..column_names.len() {
                values.push(TimeApp::new(Type::App, column_names[n].clone(), date, 0));
            }
        }
    }

    Ok(values)
}

