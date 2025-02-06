use rusqlite::{params, Connection, Result};

fn main() {
    println!("Hello, world!");
}

fn main2() -> Result<()> {
    // Create a database and table
    create_database()?;

    // Insert users
    insert_user("Alice", 30)?;
    insert_user("Bob", 25)?;

    // Query users
    println!("Users in database:");
    query_users()?;

    // Update a user
    update_user(1, 35)?;

    // Delete a user
    delete_user(2)?;

    Ok(())
}


fn create_database() -> Result<()> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn = Connection::open("time_app.db")?;

    // Create a table named users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [], // No parameters needed
    )?;

    println!("Database and table created successfully.");
    Ok(())
}

fn insert_user(name: &str, age: i32) -> Result<()> {
    let conn = Connection::open("app_database.db")?;

    // Insert a new user
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params![name, age], // Bind parameters
    )?;

    println!("User inserted successfully.");
    Ok(())
}

fn query_users() -> Result<()> {
    let conn = Connection::open("app_database.db")?;

    // Retrieve data from users table
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Iterate over the retrieved rows
    for user in user_iter {
        println!("{:?}", user?);
    }

    Ok(())
}

// Define a struct to map query results
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn update_user(id: i32, new_age: i32) -> Result<()> {
    let conn = Connection::open("app_database.db")?;

    // Update user's age
    conn.execute(
        "UPDATE users SET age = ?1 WHERE id = ?2",
        params![new_age, id],
    )?;

    println!("User updated successfully.");
    Ok(())
}

fn delete_user(id: i32) -> Result<()> {
    let conn = Connection::open("app_database.db")?;

    // Delete a user by ID
    conn.execute(
        "DELETE FROM users WHERE id = ?1",
        params![id],
    )?;

    println!("User deleted successfully.");
    Ok(())
}

