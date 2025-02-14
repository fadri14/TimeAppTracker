use argh::FromArgs;

mod frontend;
mod backend;

#[derive(FromArgs)]
/// CLI to track usage times
struct Params {
    /// launch the interface
    #[argh(switch, short = 'i')]
    int: bool,

    /// launch update
    #[argh(switch, short = 'u')]
    update: bool,

    /// add a application
    #[argh(option, short = 'a')]
    add: Option<String>,

    /// delete a application
    #[argh(option, short = 'd')]
    del: Option<String>,
}

fn main() {
    let param: Params = argh::from_env();

    if let Some(name) = param.add {
        backend::add_app(name).expect("Unable to update data in database");
    }

    if let Some(name) = param.del {
        backend::del_app(name).expect("Unable to update data in database");
    }

    if param.update {
        backend::update().expect("Unable to update data in database");
    }

    if param.int {
        frontend::interface().expect("Unable to retrieve data from database");
    }
}

