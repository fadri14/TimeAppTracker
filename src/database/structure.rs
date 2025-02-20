use chrono::NaiveDate;

const NUMBER_MINUTES_IN_HOUR: u16 = 60;

pub enum Type {
    Day,
    App(String)
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
    name: String,
    time: Time,
    pub date: NaiveDate,
    min_total: u16,
}

impl TimeApp {
    pub fn new(name: String, date: NaiveDate, mins: u16) -> TimeApp {
        let name = name[1..name.len()-1].to_string();
        TimeApp { name, time : Time::new(mins), date, min_total : mins}
    }
}

pub struct ListTimeApp {
    values: Vec<TimeApp>,
    type_data: Type,
    date: NaiveDate,
}

impl ListTimeApp {
    pub fn new(type_data: Type, mut values: Vec<TimeApp>, date: NaiveDate) -> ListTimeApp {
        values.sort_unstable_by_key(|item| (item.min_total));
        values.reverse();
        ListTimeApp{ type_data, values : values , date }
    }
}

impl std::fmt::Display for ListTimeApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.type_data {
            Type::Day => {
                let mut output = String::new();
                output.push_str(&format!("\tApplication time for {} :\n", self.date));
                for v in &self.values {
                    output.push_str(&format!("{} : {}\n", v.name, v.time));
                }

                return write!(f, "{}", output);
            }
            Type::App(name) => {
                let mut output = String::new();
                output.push_str(&format!("\tTime for {} :\n", name));
                for v in &self.values {
                    output.push_str(&format!("{} : {}\n", v.date, v.time));
                }
                
                output.push_str(&format!("\n\tStats of time for {} :\n{}\n", name, Stat::new(&self.values)));

                return write!(f, "{}", output);
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
    pub fn new(values: &Vec<TimeApp>) -> Stat {
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

        return Stat { max : Time::new(max), min : Time::new(min), mean : Time::new(count / values.len() as u16) };
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Max : {}\nMin : {}\nMean: {}", self.max, self.min, self.mean);
    }
}

