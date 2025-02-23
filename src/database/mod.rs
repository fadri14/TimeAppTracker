use rusqlite::{Connection, Result, params};
use chrono::{Duration, NaiveDate};
use notify_rust::Notification;

mod backend;
mod structure;

use backend::*;
use structure::*;

const DEFAULT_NUMBER_DAYS_SAVED: u16 = 28;

struct Settings {
    state: String,
    storage_size: u16,
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\tSettings :\nState : {}\nStorage size : {}", self.state, self.storage_size)
    }
}

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Result<Database> {
        let conn = Connection::open(get_path_bdd())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS time (
                date DATE PRIMARY KEY,
                pc INTEGER DEFAULT 0
            )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                attribute TEXT PRIMARY KEY,
                value TEXT
            )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notification (
                app TEXT PRIMARY KEY,
                time INTEGER
            )",
            (),
        )?;

        Ok(Database{ conn })
    }

    pub fn update(&self) -> Result<()> {
        if self.get_settings()?.state == "on" {
            self.delete_old_data()?;
            self.increment_time()?;
        }

        Ok(())
    }

    fn increment_time(&self) -> Result<()> {
        let column_names = self.get_column_name()?;
        let mut values = self.get_values()?;

        if column_names.len() != values.len() {
            panic!("The number of columns and data are not the same.");
        }

        self.conn.execute("DELETE FROM time WHERE date = CURRENT_DATE", (),)?;

        update_values(&column_names, &mut values);
        self.check_notif(&column_names, &values)?;

        let query = format_query(column_names, values);
        self.conn.execute(&query, [])?;

        Ok(())
    }

    fn delete_old_data(&self) -> Result<()> {
        let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
        if let Some(value) = self.get_attribute("storage_size")? {
            storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
        }

        self.conn.execute(
            "DELETE FROM time WHERE JULIANDAY(DATE()) - JULIANDAY(date) > ?1",
            ((&storage_size),),)?;

        Ok(())
    }

    fn get_column_name(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(time)")?;

        let column_names = stmt.query_map([], |row| {
            row.get::<_, String>(1)
        })?;

        let mut names = Vec::new();
        for name in column_names {
            names.push(name?);
        }
        names.remove(0);
        Ok(names)
    }

    fn get_values(&self) -> Result<Vec<u16>> {
        let mut stmt = self.conn.prepare("SELECT * FROM time WHERE date = CURRENT_DATE")?;

        let column_count = stmt.column_count();

        let mut rows = stmt.query_map([], |row| {
            let mut values: Vec<u16> = Vec::new();

            for i in 1..column_count {
                values.push(row.get(i)?);
            }

            Ok(values)
        })?;

        if let Some(Ok(values)) = rows.next() {
            return Ok(values);
        }

        Ok(vec![0 ; column_count-1])
    }

    pub fn add_app(&self, name: String) -> Result<()> {
        if name != "date" && !self.contain_names(&name)? {
            let query = format!("ALTER TABLE time ADD COLUMN [{}] INTEGER DEFAULT 0", &name);
            self.conn.execute(&query, [])?;

            return Ok(());
        }

        eprintln!("Error : The app you want to add is already present");
        Ok(())
    }

    pub fn del_app(&self, name: String) -> Result<()> {
        if name == "date" || name == SCREENTIME {
            eprintln!("Error : You cannot delete the {} column", name);
            return Ok(());
        }

        if self.contain_names(&name)? {
            let query = format!("ALTER TABLE time DROP [{}]", &name);
            self.conn.execute(&query, [])?;
            self.del_notif(&name)?;
            return Ok(());
        }

        eprintln!("Error : The application you want to delete does not exist");
        Ok(())
    }

    fn contain_names(&self, name: &String) -> Result<bool> {
        let column_names = self.get_column_name()?;
        Ok(column_names.contains(name))
    }

    fn get_settings(&self) -> Result<Settings> {
        let mut state = String::from("on");
        if let Some(value) = self.get_attribute("state")? {
            state = value;
        }

        let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
        if let Some(value) = self.get_attribute("storage_size")? {
            storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
        }

        Ok(Settings { state, storage_size })
    }

    pub fn display_settings(&self) -> Result<()> {
        println!("{}", self.get_settings()?);
        Ok(())
    }

    fn get_attribute(&self, name: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE attribute = ?1")?;

        let mut rows = stmt.query_map([&name], |row| {
            Ok(row.get(0))
        })?;

        if let Some(Ok(Ok(value))) = rows.next() {
            return Ok(Some(value))
        }

        Ok(None)
    }

    pub fn change_settings(&self, name: &str, value: &str) -> Result<()> {
        self.conn.execute("DELETE FROM settings WHERE attribute = ?1", ((&name),),)?;
        self.conn.execute("INSERT INTO settings (attribute, value) VALUES (?1, ?2)", (&name, &value),)?;
        Ok(())
    }

    pub fn switch_state(&self) -> Result<()> {
        match self.get_attribute("state")? {
            Some(value) if value == "on" => self.change_settings("state", "off")?,
            _ => self.change_settings("state", "on")?
        }

        Ok(())
    }

    fn get_time_day(&self, date: NaiveDate) -> Result<Vec<TimeApp>> {
        let column_names = self.get_column_name()?;

        let mut stmt = self.conn.prepare("SELECT * FROM time WHERE date = ?1")?;
        let mut rows = stmt.query_map(params![date.to_string()], |row| {
            let mut values: Vec<TimeApp> = Vec::new();
            for (i, name) in column_names.iter().enumerate() {
                values.push(TimeApp::new(name.clone(), date, row.get::<_, u16>(i+1)?))
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
                for name in &column_names {
                    values.push(TimeApp::new(name.clone(), date, 0));
                }
            }
        }

        Ok(values)
    }

    fn get_time_app(&self, name: &str, date: NaiveDate, number_days: u16) -> Result<Vec<TimeApp>> {
        let query = format!(
            "SELECT date, [{}] FROM time WHERE date <= '{}' and date >= DATE('{}', '-{} days') ORDER BY date DESC",
            name, date, date, number_days);
        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            let date = NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d").expect("Unable to retrieve a date");
            Ok(TimeApp::new(SCREENTIME.to_string(), date, row.get::<_, u16>(1)?))
        })?;

        let mut values: Vec<TimeApp> = Vec::new();
        for row in rows.flatten() {
            values.push(row);
            // if let Ok(time) = row {
            // values.push(time);
            // }
        }

        for i in 0..number_days {
            let deadline = date - Duration::days(i as i64);
            if i == values.len() as u16 || values[i as usize].date != deadline {
                values.insert(i as usize, TimeApp::new(SCREENTIME.to_string(), deadline, 0));
            }
        }

        Ok(values)
    }

    pub fn print_day_data(&self, date: NaiveDate) -> Result<()> {
        let values = ListTimeApp::new(Type::Day, self.get_time_day(date)?, date);
        println!("{values}");
        Ok(())
    }

    pub fn print_app_data(&self, name: String, date: NaiveDate, number_days: u16, reverse: bool) -> Result<()> {
        if ! self.contain_names(&name)? {
            eprintln!("Error : This application is not followed");
            return Ok(());
        }

        let mut values = self.get_time_app(&name, date, number_days)?;
        if reverse {
            values.reverse();
        }

        let values = ListTimeApp::new(Type::App(name.clone()), values, date);
        println!("{values}");
        Ok(())
    }

    pub fn add_notif(&self, name: &String, time: u16) -> Result<()> {
        if ! self.contain_names(name)? {
            eprintln!("Error : This application is not followed");
            return Ok(());
        }

        self.del_notif(name)?;
        self.conn.execute("INSERT INTO notification (app, time) VALUES (?1, ?2)", (name, &time),)?;

        Ok(())
    }

    pub fn del_notif(&self, name: &str) -> Result<()> {
        self.conn.execute("DELETE FROM notification WHERE app = ?1", ((name),),)?;
        Ok(())
    }

    fn check_notif(&self, names: &[String], values: &[u16]) -> Result<()> {
        for (i, name) in names.iter().enumerate() {
            let mut stmt = self.conn.prepare(&format!("SELECT 1 FROM notification WHERE app = '{}' AND time = {} LIMIT 1", &name, values[i]))?;
            let mut rows = stmt.query_map(params![], |_| {
                Notification::new()
                    .summary(&format!("Time passed for {}", &name))
                    .body(&format!("It has been {} for you to use {}. You have exceeded the set limit", Time::new(values[i]), &name))
                    .appname("Time App Tracker")
                    .show().expect("Impossible d'envoyer de notification");
                Ok(())
            })?;

            while let Some(Ok(())) = rows.next() {}
        }
        Ok(())
    }

    pub fn print_notif(&self) ->Result<()> {
        let mut stmt = self.conn.prepare("SELECT app, time FROM notification")?;
        let mut rows = stmt.query_map(params![], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, u16>(1)?))
        })?;

        println!("List of notifications :");
        while let Some(Ok((app, time))) = rows.next() {
            println!("{} => {}", app, Time::new(time));
        }
        println!();

        Ok(())
    }
}

