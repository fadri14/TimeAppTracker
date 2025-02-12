use crate::backend;
use rusqlite::{params, Connection, Result};
use chrono::{Duration, Utc, NaiveDate, Datelike, Weekday};

struct Time {
    hour: i32,
    min: i32,
    date: NaiveDate,
}

impl Time {
    fn new(date: NaiveDate, mins: i32) -> Time {
        Time { hour : mins / 60, min : mins % 60, date}
    }

    fn get_day(&self) -> String {
        match self.date.weekday() {
            Weekday::Mon => String::from("lundi   "),
            Weekday::Tue => String::from("mardi   "),
            Weekday::Wed => String::from("mercredi"),
            Weekday::Thu => String::from("jeudi   "),
            Weekday::Fri => String::from("vendredi"),
            Weekday::Sat => String::from("samedi  "),
            Weekday::Sun => String::from("dimanche"),
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.hour == 0 {
            return write!(f, "Le {} {} : {}m", self.get_day(), self.date.format("%d-%m-%Y"), self.min);
        }
        return write!(f, "Le {} {} : {}h{}", self.get_day(), self.date.format("%d-%m-%Y"), self.hour, self.min);
    }
}

pub fn gui() -> Result<()>{
    let conn = backend::connect_database()?;

    let values = get_time_main(&conn, 0)?;

    println!("\tTemps du pc: ");
    for v in values {
        println!("{}", v);
    }

    println!("");
    get_time_apps(&conn)?;

    Ok(())
}

fn get_time_main(conn: &Connection, nbr_week: i32) -> Result<Vec<Time>> {
    let nbr_week = if nbr_week < 0 || nbr_week > 3 { 0 } else { nbr_week };

    let week = Utc::now().date_naive() - Duration::days((7 * nbr_week).into());

    let mut stmt = conn.prepare("SELECT date, main FROM time WHERE date <= ?1 and date >= DATE(?1, '-7 days') ORDER BY date DESC")?;
    let rows = stmt.query_map(params![week.to_string()], |row| {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d").expect("Impossible de récupérer une date");
        Ok(Time::new(date, row.get::<_, i32>(1)?))
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
            values.insert(i, Time::new(date, 0));
        }
    }

    Ok(values)
}

fn get_time_apps(conn: &Connection) -> Result<()> {
    //let column_names = backend::get_column_name(&conn)?;
//
    //// Préparer et exécuter la requête
    //let mut stmt = conn.prepare("SELECT * FROM time WHERE date >= DATE('now', '-7 days') ORDER BY date ASC")?;
    //let rows = stmt.query_map([], |row| {
        //let mut values = Vec::new();
        //for i in 2..=column_names.len() {
            //values.push(row.get::<_, i32>(i)?);
        //}
        //Ok(values)
    //})?;
//
    //println!("Temps des applications:");
    //// Parcourir les résultats
    //for row in rows {
        //let values: Vec<i32> = row?;
        //println!("{:?}", values);
    //}

    Ok(())
}

