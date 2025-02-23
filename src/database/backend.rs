use std::env;
use std::process::Command;

pub const SCREENTIME: &str = "pc";

pub fn get_path_bdd() -> String {
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

pub fn app_running(name: &str) -> bool {
    if name == SCREENTIME {
        return true;
    }

    let output = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .expect("Failed to execute pgrep command");

    !output.stdout.is_empty()
}

pub fn update_values(names: &[String], values: &mut [u16]) {
    let mut index = 0;
    while index < names.len() {
        if app_running(&names[index]) {
            values[index] += 1;
        }

        index += 1;
    }
}

pub fn format_query(column_names: Vec<String>, values: Vec<u16>) -> String {
    let mut names_query = String::new();
    let mut values_query = String::new();

    let mut names: Vec<String> = Vec::new();
    for mut n in column_names {
        n.insert(0, '[');
        n.push(']');
        names.push(n);
    }

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

