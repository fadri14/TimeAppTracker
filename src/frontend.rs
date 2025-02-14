use crate::backend;
use rusqlite::{params, Connection, Result};
use chrono::{Duration, Utc, NaiveDate, Datelike};

enum Type {
    Main,
    App
}

struct Time {
    hour: i32,
    min: i32,
}

impl Time {
    fn new(mins: i32) -> Time {
        if mins <= 0 {
            return Time { hour : 0, min : 0 };
        }
        return Time { hour : mins / 60, min : mins % 60 };
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.hour == 0 {
            return write!(f, "{}m", self.min);
        }
        return write!(f, "{}h{}", self.hour, self.min);
    }
}

struct TimeApp {
    type_data: Type,
    name: String,
    time: Time,
    date: NaiveDate,
    min_total: i32,
}

impl TimeApp {
    fn new(type_data: Type, name: String, date: NaiveDate, mins: i32) -> TimeApp {
        let start = 1;
        let end = name.len()-1;
        let name = name[start..end].to_string();
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
        //for v in values[1..].into_iter() {
            count += values[i].min_total;

            if min > values[i].min_total {
                min = values[i].min_total;
            }

            if max < values[i].min_total {
                max = values[i].min_total;
            }
        }

        return Stat { max : Time::new(max), min : Time::new(min), mean : Time::new(count / (values.len() + 1) as i32) };
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Max : {}\nMin : {}\nMean: {}", self.max, self.min, self.mean);
    }
}

pub fn interface() -> Result<()>{
    let conn = backend::connect_database()?;

    let values = get_time_main(&conn, 0)?;

    println!("\tPC time : ");
    for v in &values {
        println!("{}", v);
    }

    println!("\n\tStats of PC time :\n{}", Stat::new(values));

    println!("");
    let date = Utc::now().date_naive() - Duration::days(0);
    let values = get_time_apps(&conn, date)?;
    if values.len() > 0 {
        println!("\tApplication time for {} : ", date);
        for v in values {
            println!("{}", v);
        }
    }

    Ok(())
}

fn get_time_main(conn: &Connection, nbr_week: i32) -> Result<Vec<TimeApp>> {
    let nbr_week = if nbr_week < 0 || nbr_week > 3 { 0 } else { nbr_week };

    let week = Utc::now().date_naive() - Duration::days((7 * nbr_week).into());

    let mut stmt = conn.prepare("SELECT date, main FROM time WHERE date <= ?1 and date >= DATE(?1, '-7 days') ORDER BY date DESC")?;
    let rows = stmt.query_map(params![week.to_string()], |row| {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d").expect("Unable to retrieve a date");
        Ok(TimeApp::new(Type::Main, "main".to_string(), date, row.get::<_, i32>(1)?))
    })?;

    let mut values: Vec<TimeApp> = Vec::new();
    for row in rows {
        if let Ok(time) = row {
            values.push(time);
        }
    }

    for i in 0..7 {
        let date = week - Duration::days(i as i64);
        if i == values.len() || values[i].date != date {
            values.insert(i, TimeApp::new(Type::Main, "main".to_string(), date, 0));
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
            values.push(TimeApp::new(Type::App, column_names[i].clone(), date, row.get::<_, i32>(i+1)?))
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

