use std::process::Command;
use std::env;
use rusqlite::{Connection, Result};

const DEFAULT_NUMBER_DAYS_SAVED: u16 = 28;

struct Settings {
    state: String,
    storage_size: u16,
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "State : {}\nStorage size : {}", self.state, self.storage_size);
    }
}

pub fn update() -> Result<()> {
    if get_settings()?.state == String::from("on") {
        let conn = connect_database()?;
        delete_old_data(&conn)?;
        increment_time(&conn)?;
    }

    Ok(())
}

fn get_path_bdd() -> String {
    // If the HOME variable is well defined then we store the database in the personal folder.
    // Otherwise, it stored in the current directory
    let mut path = String::new();
    match env::var("HOME") {
        Ok(val) if val.contains("/home") => {
            path.push_str(&val);
            path.push_str("/.time_app_tracker.db")
        }
        _ => path.push_str(".time_app_tracker.db"),
    }

    //path
    String::from("time_app_tracker.db")
}

pub fn connect_database() -> Result<Connection> {
    let conn = Connection::open(get_path_bdd())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (
            date DATE PRIMARY KEY,
            main INTEGER DEFAULT 0
        )",
        (),
    )?;

    Ok(conn)
}

fn increment_time(conn: &Connection) -> Result<()> {
    let column_names = get_column_name(&conn)?;
    let mut values = get_values(&conn)?;

    if column_names.len() != values.len() {
        panic!("The number of columns and data are not the same.");
    }

    conn.execute("DELETE FROM time WHERE date = CURRENT_DATE", (),)?;

    update_values(&column_names, &mut values);

    let query = format_query(column_names, values);
    conn.execute(&query, [])?;

    Ok(())
}

fn delete_old_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            attribute TEXT PRIMARY KEY,
            value TEXT
        )",
        (),
    )?;

    let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
    if let Some(value) = get_attribute(&conn, String::from("storage_size"))? {
        storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
    }

    let query = format!("DELETE FROM time WHERE JULIANDAY(DATE()) - JULIANDAY(date) > {}", storage_size);
    conn.execute(&query, [])?;

    Ok(())
}

pub fn get_column_name(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("PRAGMA table_info(time)")?;

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

fn get_values(conn: &Connection) -> Result<Vec<u16>> {
    let mut stmt = conn.prepare("SELECT * FROM time WHERE date = CURRENT_DATE")?;

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

fn update_values(names: &Vec<String>, values: &mut Vec<u16>) {
    let mut index = 0;
    while index < names.len() {
        if app_running(&names[index]) {
            values[index] += 1;
        }

        index += 1;
    }
}

fn format_query(names: Vec<String>, values: Vec<u16>) -> String {
    let mut names_query = String::new();
    let mut values_query = String::new();

    let mut index = 0;
    while index < names.len() {
        names_query.push_str(&names[index]);
        names_query.push(',');

        values_query.push_str(&(values[index].to_string()));
        values_query.push(',');

        index += 1;
    }

    names_query.pop();
    values_query.pop();

    format!("INSERT INTO time (date, {}) VALUES (CURRENT_DATE, {})", names_query, values_query)
}

fn app_running(name: &String) -> bool {
    let name = &name[1..name.len()-1];

    if name == "main" {
        return true;
    }

    let output = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .expect("Failed to execute pgrep command");

    !output.stdout.is_empty()
}

pub fn add_app(name: String) -> Result<()> {
    let conn = connect_database()?;

    if name != "date" && !contain_names(&conn, &name)? {
        let query = format!("ALTER TABLE time ADD COLUMN [{}] INTEGER DEFAULT 0", &name);
        conn.execute(&query, [])?;
        return Ok(());
    }

    panic!("the app you want to add is already present");
}

pub fn del_app(name: String) -> Result<()> {
    if name == "date" || name == "main" {
        panic!("You cannot delete the {} column", name);
    }

    let conn = connect_database()?;

    if contain_names(&conn, &name)? {
        let query = format!("ALTER TABLE time DROP [{}]", &name);
        conn.execute(&query, [])?;
        return Ok(());
    }

    panic!("The application you want to delete does not exist");
}

fn contain_names(conn: &Connection, name: &String) -> Result<bool> {
    let column_names = get_column_name(&conn)?;

    for n in column_names {
        if name == &n[1..n.len()-1] {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_settings() -> Result<Settings> {
    let conn = Connection::open(get_path_bdd())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            attribute TEXT PRIMARY KEY,
            value TEXT
        )",
        (),
    )?;

    let mut state = String::from("on");
    if let Some(value) = get_attribute(&conn, String::from("state"))? {
        state = value;
    }

    let mut storage_size = DEFAULT_NUMBER_DAYS_SAVED;
    if let Some(value) = get_attribute(&conn, String::from("storage_size"))? {
        storage_size = value.parse::<u16>().unwrap_or(DEFAULT_NUMBER_DAYS_SAVED);
    }

    Ok(Settings {
        state : state,
        storage_size : storage_size
    })
}

pub fn display_settings() -> Result<()> {
    println!("{}", get_settings()?);
    Ok(())
}

fn get_attribute(conn: &Connection, name: String) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE attribute = ?1")?;

    let mut rows = stmt.query_map([&name], |row| {
        Ok(row.get(0))
    })?;

    if let Some(Ok(Ok(value))) = rows.next() {
        return Ok(Some(value))
    }

    Ok(None)
}

pub fn change_settings(name: String, value: String) -> Result<()> {
    let conn = Connection::open(get_path_bdd())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            attribute TEXT PRIMARY KEY,
            value TEXT
        )",
        (),
    )?;

    set_attribute(&conn, name, value)?;
    Ok(())
}

pub fn set_attribute(conn: &Connection, name: String, value: String) -> Result<()> {
    conn.execute("DELETE FROM settings WHERE attribute = ?1", ((&name),),)?;
    conn.execute("INSERT INTO settings (attribute, value) VALUES (?1, ?2)", (&name, &value),)?;
    Ok(())
}

pub fn switch_state() -> Result<()> {
    let conn = Connection::open(get_path_bdd())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            attribute TEXT PRIMARY KEY,
            value TEXT
        )",
        (),
    )?;

    match get_attribute(&conn, String::from("state"))? {
        Some(value) if value == String::from("on") => set_attribute(&conn, String::from("state"), String::from("off"))?,
        _ => set_attribute(&conn, String::from("state"), String::from("on"))?
    }

    Ok(())
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

