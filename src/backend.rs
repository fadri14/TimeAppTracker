use rusqlite::{params, Connection, Result};

pub fn update() -> Result<()> {
    println!("mode update");
    create_database()?;
    increment_time()?;

    Ok(())
}

fn create_database() -> Result<()> {
    // Se connecte à la base de données, elle est créée si elle n'existe pas
    let conn = Connection::open("time_app.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (
            date TEXT PRIMARY KEY,
            main INTEGER DEFAULT 0
        )",
        [],
    )?;

    println!("La base de données est créée");
    Ok(())
}

fn increment_time() -> Result<()> {
    // Supprimer les lignes qui dates de plus de 28 jours
    //
    // Récupérer le nom des colonnes
    // Récupérer la ligne d'aujourd'hui si elle existe
    // Sinon créer une liste de 0 le taille du nombre de colonne moins 1
    // Pour chaque colonne sauf la première, regarder si le programme est en cours
    // Incrémenter la valeur de l'appli si elle tourne
    // Écrire cette ligne dans la bdd

    Ok(())
}

