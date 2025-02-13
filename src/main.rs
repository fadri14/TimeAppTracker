use std::env;

mod frontend;
mod backend;

fn main() {
    let mut args = env::args();

    args.next();
    match args.next() {
        Some(m) if m == "update" => backend::update().expect("Unable to update data in database"),
        _ => frontend::gui().expect("Unable to retrieve data from database"),
    }
}

