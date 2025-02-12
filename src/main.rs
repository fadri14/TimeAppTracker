use std::env;

mod frontend;
mod backend;

fn main() {
    let mut args = env::args();

    args.next();
    match args.next() {
        Some(m) if m == "gui" => frontend::gui().expect("erreur de l'interface graphique"),
        Some(m) if m == "update" => backend::update().expect("erreur de base de données"),
        _ => eprintln!("il faut un argument: [gui|update]"),
    }
}

