use argh::FromArgs;

mod frontend;
mod backend;

#[derive(FromArgs)]
/// CLI to track usage times
struct Params {
    /// pause the timer
    #[argh(switch, short = 'p')]
    pause: bool,

    /// get the status of timer
    #[argh(switch, short = 's')]
    status: bool,

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
    let mut flag = true;

    if param.pause {
        backend::state(backend::State::Change).expect("pause : Unable to work with database");
        flag = false;
    }

    if param.status {
        if backend::state(backend::State::Get).expect("status : Unable to work with database") {
            println!("The timer is on");
        }
        else {
            println!("The timer is off");
        }

        flag = false;
    }

    if let Some(name) = param.add {
        backend::add_app(name).expect("add : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.del {
        backend::del_app(name).expect("del : Unable to work with database");
        flag = false;
    }

    if param.update {
        backend::update().expect("update : Unable to work with database");
        flag = false;
    }

    if param.int {
        frontend::interface().expect("int : Unable to work with database");
        flag = false;
    }

    if flag {
        println!("run `time_app_tracker --help` for help");
    }
}

