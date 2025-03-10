use argh::FromArgs;
use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};

mod database;

use database::Database;

const VERSION_NUMBER: &str = "v0.1.7";

#[derive(PartialEq)]
enum TypeRequest {
    Day,
    App,
}

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

    /// enables notification mode for an application
    #[argh(option)]
    add_notif: Option<String>,

    /// indicates the time in minutes before a notification is sent
    #[argh(option)]
    notif_time: Option<u16>,

    /// removes notification functionality for an application
    #[argh(option)]
    del_notif: Option<String>,

    /// displays the list of notifications
    #[argh(switch)]
    print_notif: bool,

    /// launch update
    #[argh(switch, short = 'u')]
    update: bool,

    /// add a application
    #[argh(option)]
    add: Option<String>,

    /// delete a application
    #[argh(option)]
    del: Option<String>,

    /// to retrieve data either for a day's info with [daydata] or an application's info with [app-<name>]
    #[argh(option, short = 'q')]
    query: Option<String>,

    /// select the date of the retrieved data, foramt : YYYY-mm-dd. you can also use keywords such as yesterday, last_week or a day of the week (monday…).
    #[argh(option)]
    date: Option<String>,

    /// select the number of day of the retrieved data
    #[argh(option, short = 'n', default = "0")]
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
        if mode == "on" || mode == "off" {
            database
                .change_settings("state", &mode)
                .expect("state : Unable to work with database");
        } else if mode == "switch" {
            database
                .switch_state()
                .expect("state : Unable to work with database");
        } else {
            eprintln!("Error : there are only three possible modes [on|off|switch]");
        }
        flag = false;
    }

    if let Some(number) = param.storage {
        database
            .change_settings("storage_size", &number.to_string())
            .expect("storage : Unable to work with database");
        flag = false;
    }

    if param.settings {
        database
            .display_settings()
            .expect("settings : Unable to work with database");
        flag = false;
    }

    match (param.add_notif, param.notif_time) {
        (Some(name), Some(time)) => {
            database
                .add_notif(&name, time)
                .expect("notif_app : Unable to work with database");
            flag = false;
        }
        (None, None) => (),
        _ => {
            eprintln!("Error : you must use the arguments [--notif_app] and [--notif_time] at the same time");
            flag = false;
        }
    }

    if let Some(name) = param.del_notif {
        database
            .del_notif(&name)
            .expect("del_notif_app : Unable to work with database");
        flag = false;
    }

    if param.print_notif {
        database
            .print_notif()
            .expect("print_notif : Unable to work with database");
        flag = false;
    }

    if param.update {
        database
            .update()
            .expect("update : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.add {
        database
            .add_app(name)
            .expect("add : Unable to work with database");
        flag = false;
    }

    if let Some(name) = param.del {
        database
            .del_app(name)
            .expect("del : Unable to work with database");
        flag = false;
    }

    if let Some(query) = param.query {
        if query == "daydata" {
            let (date, number) = get_value_or_default(TypeRequest::Day, param.date, param.number);
            database
                .print_day_data(date, number)
                .expect("daydata : Unable to work with database");
        } else if query.len() >= 5 && query[0..4] == *"app-" {
            let (date, number) = get_value_or_default(TypeRequest::App, param.date, param.number);
            database
                .print_app_data(query[4..].to_string(), date, number, param.reverse)
                .expect("app : Unable to work with database");
        } else {
            eprintln!("Query error. Please use [daydata] or [app-<name>] as query parameter");
        }
        flag = false;
    }

    if flag {
        println!("run `time_app_tracker --help` for help");
    }
}

fn get_value_or_default(
    type_request: TypeRequest,
    date: Option<String>,
    number: u16,
) -> (NaiveDate, u16) {
    let date = date.clone().unwrap_or_else(|| String::from("today"));
    let mut date_res = Utc::now().date_naive();

    let mut number_res: u16;
    if number == 0 {
        match type_request {
            TypeRequest::Day => number_res = 1,
            TypeRequest::App => number_res = 10,
        }
    } else {
        number_res = number;
    }

    match date.to_lowercase().as_str() {
        "today" | "t" => (),
        "yesterday" | "y" => date_res = Utc::now().date_naive() - Duration::days(1),
        "monday" | "mon" => date_res = weekday_to_date(Weekday::Mon),
        "tuesday" | "tue" => date_res = weekday_to_date(Weekday::Tue),
        "wednesday" | "wed" => date_res = weekday_to_date(Weekday::Wed),
        "thursday" | "thu" => date_res = weekday_to_date(Weekday::Thu),
        "friday" | "fri" => date_res = weekday_to_date(Weekday::Fri),
        "saturday" | "sat" => date_res = weekday_to_date(Weekday::Sat),
        "sunday" | "sun" => date_res = weekday_to_date(Weekday::Sun),
        "last_week" | "lw" => {
            date_res = weekday_to_date(Weekday::Sun);
            if number == 0 && type_request == TypeRequest::App {
                number_res = 7
            };
        }
        d => {
            if let Ok(date_parse) = NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                date_res = date_parse;
            }
        }
    }

    (date_res, number_res)
}

fn weekday_to_date(day: Weekday) -> NaiveDate {
    let today = Utc::now().date_naive();

    let mut days_to_subtract = match today.weekday().num_days_from_monday() {
        n if n >= day.num_days_from_monday() => n - day.num_days_from_monday(),
        n => n + 7 - day.num_days_from_monday(),
    };

    if days_to_subtract == 0 {
        days_to_subtract = 7;
    }

    today - Duration::days(days_to_subtract as i64)
}
