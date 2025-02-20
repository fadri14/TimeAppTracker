use argh::FromArgs;
use chrono::{Utc, NaiveDate};

mod database;

use database::Database;

const VERSION_NUMBER: &str = "v0.1.6";

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
/// CLI to track usage times for pc and applications
struct Params {
    /// to get the current version number
    #[argh(switch, short = 'v')]
    version: bool,

    /// switch between on and off state
    #[argh(option)]
    state: Option<String>,

    /// change the size of the storage
    #[argh(option)]
    storage: Option<u16>,

    /// get the settings of this application
    #[argh(switch, short = 's')]
    settings: bool,

    ///// enables notification mode for an application
    //#[argh(option)]
    //notif_app: Option<String>,
//
    ///// specifies the time before receiving a notification
    //#[argh(option)]
    //notif_time: Option<String>,

    /// launch update
    #[argh(switch, short = 'u')]
    update: bool,

    /// add a application
    #[argh(option)]
    add: Option<String>,

    /// delete a application
    #[argh(option)]
    del: Option<String>,

    /// recover data from a day
    #[argh(switch, short = 'd')]
    daydata: bool,

    /// retrieve data from an application
    #[argh(option, short = 'a')]
    app: Option<String>,

    /// select the date of the retrieved data, foramt : YYYY-mm-dd
    #[argh(option, default = "Utc::now().date_naive()")]
    date: NaiveDate,

    /// select the number of day of the retrieved data
    #[argh(option, short = 'n', default = "7")]
    number: u16,

    /// inverts the result for an application
    #[argh(switch, short = 'r')]
    reverse: bool,
}

fn main() {
    let param: Params = argh::from_env();
    let mut flag = true;

    let database = Database::new().expect("Unable to work with database");

    if param.version {
        println!("current version : {}", VERSION_NUMBER);
        flag = false;
    }

    if let Some(mode) = param.state {
        if mode == String::from("on") || mode == String::from("off") {
            database.change_settings(String::from("state"), mode).expect("state : Unable to work with database");
        }
        else if mode == String::from("switch") {
            database.switch_state().expect("state : Unable to work with database");
        }
        else {
            println!("Error : there are only three possible modes [on|off|switch]");
        }
        flag = false;
    }

    if let Some(number) = param.storage {
        database.change_settings(String::from("storage_size"), number.to_string()).expect("storage : Unable to work with database");
        flag = false;
    }

    if param.settings {
        database.display_settings().expect("settings : Unable to work with database");
        flag = false;
    }

    //if let Some(name) = param.notif_app {
    //match (param.notif_app, param.notif_time) {
        //(Some(name), Some(time)) => {
        //et time = DateTime::parse_from_str(time, "%H:%M").expect("Wrong duration syntax. Used HH:MM");
        //database.set_notif(name, time).expect("notif_app : Unable to work with database");
        //},
        //_ => println!("Error : you must use the arguments [--notif_app] and [--notif_time] at the same time"),
    //}
        //flag = false;
    //}

    if param.update {
        database.update().expect("update : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.add {
        database.add_app(name).expect("add : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.del {
        database.del_app(name).expect("del : Unable to work with database");
        flag = false;
    }

    if param.daydata {
        database.print_day_data(param.date).expect("daydata : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.app {
        database.print_app_data(name, param.date, param.number, param.reverse).expect("app : Unable to work with database");
        flag = false;
    }

    if flag {
        println!("run `time_app_tracker --help` for help");
    }
}

