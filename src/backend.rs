use std::process::Command;
use std::env;
use rusqlite::{Connection, Result};

pub fn update() -> Result<()> {
    let conn = connect_database()?;
    delete_old_data(&conn)?;
    increment_time(&conn)?;

    Ok(())
}

pub fn connect_database() -> Result<Connection> {
    // Si la variable HOME est bien définie alors on stocke la base de données dans le dossier personnelle
    // Sinon, elle stocké dans le répertoire courant
    let mut path = String::new();
    match env::var("HOME") {
        Ok(val) if val.contains("/home") => {
            path.push_str(&val);
            path.push_str("/time_app.db")
        }
        _ => path.push_str("time_app.db"),
    }

    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (
            date DATE PRIMARY KEY,
            main INTEGER DEFAULT 0,
            alacritty INTEGER DEFAULT 0,
            nvim INTEGER DEFAULT 0,
            librewolf INTEGER DEFAULT 0,
            freetube INTEGER DEFAULT 0,
            [signal-desktop] INTEGER DEFAULT 0,
            netflix INTEGER DEFAULT 0,
            xournalpp INTEGER DEFAULT 0,
            spotube INTEGER DEFAULT 0,
            nautilus INTEGER DEFAULT 0,
            [gnome-evince] INTEGER DEFAULT 0,
            calculator INTEGER DEFAULT 0,
            ranger INTEGER DEFAULT 0
        )",
        (),
    )?;

    Ok(conn)
}

fn increment_time(conn: &Connection) -> Result<()> {
    let column_names = get_colmn_name(&conn)?;
    let mut values = get_values(&conn)?;

    if column_names.len() != values.len() {
        panic!("Le nombre de colonnes et de données ne sont pas les même");
    }

    conn.execute(
        "DELETE FROM time WHERE date = CURRENT_DATE",
        (),
    )?;

    update_values(&column_names, &mut values);

    let query = format_query(column_names, values);
    println!("le requête : {}", query);
    conn.execute(&query, [])?;

    Ok(())
}

fn delete_old_data(conn: &Connection) -> Result<()> {
    // Supprimer les lignes où la date est plus ancienne que 28 jours
    conn.execute(
        "DELETE FROM time WHERE JULIANDAY(DATE()) - JULIANDAY(date) > 28",
        (),
    )?;

    Ok(())
}

fn get_colmn_name(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("PRAGMA table_info(time)")?;

    // Exécuter la requête et récupérer les résultats
    let column_info_iter = stmt.query_map([], |row| {
        // La colonne 'name' contient le nom de la colonne
        row.get::<_, String>(1)
    })?;

    // Créer un vecteur pour stocker les noms des colonnes
    let mut column_names = Vec::new();

    // Parcourir les résultats et remplir le vecteur
    for column_name in column_info_iter {
        column_names.push("[".to_string() + &column_name? + "]");
    }

    column_names.remove(0);
    Ok(column_names)
}

fn get_values(conn: &Connection) -> Result<Vec<i32>> {
    let mut stmt = conn.prepare("SELECT * FROM time WHERE date = CURRENT_DATE")?;

    let column_count = stmt.column_count();

    // Exécuter la requête et itérer sur les résultats
    let mut rows = stmt.query_map([], |row| {
        let mut values: Vec<i32> = Vec::new();

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

fn update_values(names: &Vec<String>, values: &mut Vec<i32>) {
    let mut index = 0;
    while index < names.len() {
        if app_running(&names[index]) {
            values[index] += 1;
        }

        index += 1;
    }
}

fn format_query(names: Vec<String>, values: Vec<i32>) -> String {
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
    let start = 1;
    let end = name.len()-1;
    let name = &name[start..end];

    if name == "main" {
        return true;
    }

    let output = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .expect("Échec de l'exécution de la commande pgrep");

    !output.stdout.is_empty()
}

