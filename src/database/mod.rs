use rusqlite::{Connection, Result, params};
use chrono::{Duration, NaiveDate};

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
        return write!(f, "\tSettings :\nState : {}\nStorage size : {}", self.state, self.storage_size);
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
                main INTEGER DEFAULT 0
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

        Ok(Database{ conn : conn })
    }

    pub fn update(&self) -> Result<()> {
        if self.get_settings()?.state == String::from("on") {
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

        let query = format_query(column_names, values);
        self.conn.execute(&query, [])?;

        Ok(())
    }

    fn delete_old_data(&self) -> Result<()> {
        let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
        if let Some(value) = self.get_attribute(String::from("storage_size"))? {
            storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
        }

        let query = format!("DELETE FROM time WHERE JULIANDAY(DATE()) - JULIANDAY(date) > {}", storage_size);
        self.conn.execute(&query, [])?;

        Ok(())
    }

    fn get_column_name(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(time)")?;

        let column_info_iter = stmt.query_map([], |row| {
            row.get::<_, String>(1)
        })?;

        let mut column_names = Vec::new();

        for column_name in column_info_iter {
            column_names.push("[".to_string() + &column_name? + "]");
        }

        column_names.remove(0);
        Ok(column_names)
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

        panic!("the app you want to add is already present");
    }

    pub fn del_app(&self, name: String) -> Result<()> {
        if name == "date" || name == "main" {
            panic!("You cannot delete the {} column", name);
        }

        if self.contain_names(&name)? {
            let query = format!("ALTER TABLE time DROP [{}]", &name);
            self.conn.execute(&query, [])?;
            return Ok(());
        }

        panic!("The application you want to delete does not exist");
    }

    fn contain_names(&self, name: &String) -> Result<bool> {
        let column_names = self.get_column_name()?;

        for n in column_names {
            if name == &n[1..n.len()-1] {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn get_settings(&self) -> Result<Settings> {
        let mut state = String::from("on");
        if let Some(value) = self.get_attribute(String::from("state"))? {
            state = value;
        }

        let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
        if let Some(value) = self.get_attribute(String::from("storage_size"))? {
            storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
        }

        Ok(Settings {
            state : state,
            storage_size : storage_size
        })
    }

    pub fn display_settings(&self) -> Result<()> {
        println!("{}", self.get_settings()?);
        Ok(())
    }

    fn get_attribute(&self, name: String) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE attribute = ?1")?;

        let mut rows = stmt.query_map([&name], |row| {
            Ok(row.get(0))
        })?;

        if let Some(Ok(Ok(value))) = rows.next() {
            return Ok(Some(value))
        }

        Ok(None)
    }

    pub fn change_settings(&self, name: String, value: String) -> Result<()> {
        self.conn.execute("DELETE FROM settings WHERE attribute = ?1", ((&name),),)?;
        self.conn.execute("INSERT INTO settings (attribute, value) VALUES (?1, ?2)", (&name, &value),)?;
        Ok(())
    }

    pub fn switch_state(&self) -> Result<()> {
        match self.get_attribute(String::from("state"))? {
            Some(value) if value == String::from("on") => self.change_settings(String::from("state"), String::from("off"))?,
            _ => self.change_settings(String::from("state"), String::from("on"))?
        }

        Ok(())
    }

    fn get_time_main(&self, date: NaiveDate, number_days: u16) -> Result<Vec<TimeApp>> {
        let query = format!("SELECT date, main FROM time WHERE date <= ?1 and date >= DATE(?1, '-{} days') ORDER BY date DESC", number_days);
        let mut stmt = self.conn.prepare(&query)?;
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

    fn get_time_apps(&self, date: NaiveDate) -> Result<Vec<TimeApp>> {
        let column_names = self.get_column_name()?;

        let mut stmt = self.conn.prepare("SELECT * FROM time WHERE date = ?1")?;
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

    pub fn print_main(&self, date: NaiveDate, number_days: u16) -> Result<()> {
        let values = self.get_time_main(date, number_days)?;

        println!("\tPC time : ");
        for v in &values {
            println!("{}", v);
        }

        println!("\n\tStats of PC time :\n{}", Stat::new(values));

        Ok(())
    }

    pub fn print_apps(&self, date: NaiveDate) -> Result<()> {
        let values = self.get_time_apps(date)?;
        if values.len() > 0 {
            println!("\tApplication time for {} : ", date);
            for v in values {
                println!("{}", v);
            }
        }

        Ok(())
    }
}

//pub fn set_notif(name: String, time: DateTime) -> Result<()> {
    // Se connecter à la bdd et créer la table
    // Surprimmer les lignes où il y a name pour être sûr qu'il y en a qu'un seul
    // insérer une nouvelle ligne avec name et time
    //
    // exemple :
    // notify-rust
// use notify_rust::Notification;
// Notification::new()
    // .summary("Firefox News")
    // .body("This will almost look like a real firefox notification.")
    // .icon("firefox")
    // .show()?;
// 
    // use notify_rust::{Notification, Hint};
// Notification::new()
    // .summary("Category:email")
    // .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    // .icon("thunderbird")
    // .appname("thunderbird")
    // .hint(Hint::Category("email".to_owned()))
    // .hint(Hint::Resident(true)) // this is not supported by all implementations
    // .timeout(0) // this however is
    // .show()?;
//    Ok(())
//}

