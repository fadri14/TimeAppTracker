use crate::backend;
use rusqlite::{params, Connection, Result};
use chrono::{Duration, Utc, NaiveDate, Datelike};

enum Type {
    Main,
    App
}

struct Time {
    type_data: Type,
    name: String,
    hour: i32,
    min: i32,
    date: NaiveDate,
}

impl Time {
    fn new(type_data: Type, name: String, date: NaiveDate, mins: i32) -> Time {
        let start = 1;
        let end = name.len()-1;
        let name = name[start..end].to_string();
        Time { type_data, name, hour : mins / 60, min : mins % 60, date}
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_data {
            Type::App => {
                if self.hour == 0 {
                    return write!(f, "{} : {}m", self.name, self.min);
                }
                return write!(f, "{} : {}h{}", self.name, self.hour, self.min);
            }
            Type::Main => {
                if self.hour == 0 {
                    return write!(f, "{} {} : {}m", self.date.weekday(), self.date.format("%d-%m-%Y"), self.min);
                }
                return write!(f, "{} {} : {}h{}", self.date.weekday(), self.date.format("%d-%m-%Y"), self.hour, self.min);
            }
        }
    }
}

pub fn interface() -> Result<()>{
    let conn = backend::connect_database()?;

    let values = get_time_main(&conn, 0)?;

    println!("\tPC time : ");
    for v in values {
        println!("{}", v);
    }

    println!("");
    let date = Utc::now().date_naive() - Duration::days(0);
    let values = get_time_apps(&conn, date)?;
    if values.len() > 0 {
        println!("\tApplication time for {} : ", date.format("%d-%m-%Y"));
        for v in values {
            println!("{}", v);
        }
    }

    Ok(())
}

fn get_time_main(conn: &Connection, nbr_week: i32) -> Result<Vec<Time>> {
    let nbr_week = if nbr_week < 0 || nbr_week > 3 { 0 } else { nbr_week };

    let week = Utc::now().date_naive() - Duration::days((7 * nbr_week).into());

    let mut stmt = conn.prepare("SELECT date, main FROM time WHERE date <= ?1 and date >= DATE(?1, '-7 days') ORDER BY date DESC")?;
    let rows = stmt.query_map(params![week.to_string()], |row| {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d").expect("Unable to retrieve a date");
        Ok(Time::new(Type::Main, "main".to_string(), date, row.get::<_, i32>(1)?))
    })?;

    let mut values: Vec<Time> = Vec::new();
    for row in rows {
        if let Ok(time) = row {
            values.push(time);
        }
    }

    for i in 0..7 {
        let date = week - Duration::days(i as i64);
        if i == values.len() || values[i].date != date {
            values.insert(i, Time::new(Type::Main, "main".to_string(), date, 0));
        }
    }

    Ok(values)
}

fn get_time_apps(conn: &Connection, date: NaiveDate) -> Result<Vec<Time>> {
    let column_names = backend::get_column_name(&conn)?;

    let mut stmt = conn.prepare("SELECT * FROM time WHERE date = ?1")?;
    let mut rows = stmt.query_map(params![date.to_string()], |row| {
        let mut values: Vec<Time> = Vec::new();
        for i in 1..column_names.len() {
            values.push(Time::new(Type::App, column_names[i].clone(), date, row.get::<_, i32>(i+1)?))
        }
        Ok(values)
    })?;

    let mut values: Vec<Time> = Vec::new();
    match rows.next() {
        Some(Ok(vec_times)) => {
            for t in vec_times {
                values.push(t);
            }
        }
        _ => {
            for n in 1..column_names.len() {
                values.push(Time::new(Type::App, column_names[n].clone(), date, 0));
            }
        }
    }

    Ok(values)
}

