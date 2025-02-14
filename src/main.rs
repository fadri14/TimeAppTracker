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

    /// pause the timer
    #[argh(switch, short = 'p')]
    pause: bool,
}

fn main() {
    let param: Params = argh::from_env();
    let mut flag = true;

    if let Some(name) = param.add {
        backend::add_app(name).expect("Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.del {
        backend::del_app(name).expect("Unable to work with database");
        flag = false;
    }

    if param.update {
        backend::update().expect("Unable to work with database");
        flag = false;
    }

    if param.int {
        frontend::interface().expect("Unable to work with database");
        flag = false;
    }

    if param.pause {
        backend::state(backend::State::Change).expect("Unable to work with database");
        flag = false;
    }

    if flag {
        println!("run `time_app_tracker --help` for help");
    }
}

