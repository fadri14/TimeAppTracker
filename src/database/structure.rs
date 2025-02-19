use chrono::{NaiveDate, Datelike};

const NUMBER_MINUTES_IN_HOUR: u16 = 60;

pub enum Type {
    Main,
    App
}

struct Time {
    hour: u16,
    min: u16,
}

impl Time {
    fn new(mins: u16) -> Time {
        if mins <= 0 {
            return Time { hour : 0, min : 0 };
        }
        return Time { hour : mins / NUMBER_MINUTES_IN_HOUR, min : mins % NUMBER_MINUTES_IN_HOUR };
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.hour == 0 {
            return write!(f, "{}m", self.min);
        }
        if self.min < 10 {
            return write!(f, "{}h0{}", self.hour, self.min);
        }
        return write!(f, "{}h{}", self.hour, self.min);
    }
}

pub struct TimeApp {
    type_data: Type,
    name: String,
    time: Time,
    pub date: NaiveDate,
    min_total: u16,
}

impl TimeApp {
    pub fn new(type_data: Type, name: String, date: NaiveDate, mins: u16) -> TimeApp {
        let name = name[1..name.len()-1].to_string();
        TimeApp { type_data, name, time : Time::new(mins), date, min_total : mins}
    }
}

impl std::fmt::Display for TimeApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_data {
            Type::App => {
                return write!(f, "{} : {}", self.name, self.time);
            }
            Type::Main => {
                return write!(f, "{} {} : {}", self.date.weekday(), self.date, self.time);
            }
        }
    }
}

pub struct Stat {
    max: Time,
    min: Time,
    mean: Time,
}

impl Stat {
    pub fn new(values: Vec<TimeApp>) -> Stat {
        if values.len() == 0 {
            return Stat { max : Time::new(0), min : Time::new(0), mean : Time::new(0) };
        }

        let mut count = values[0].min_total;
        let mut min = values[0].min_total;
        let mut max = values[0].min_total;

        for i in 1..values.len() {
            count += values[i].min_total;

            if min > values[i].min_total {
                min = values[i].min_total;
            }

            if max < values[i].min_total {
                max = values[i].min_total;
            }
        }

        return Stat { max : Time::new(max), min : Time::new(min), mean : Time::new(count / (values.len() + 1) as u16) };
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Max : {}\nMin : {}\nMean: {}", self.max, self.min, self.mean);
    }
}

