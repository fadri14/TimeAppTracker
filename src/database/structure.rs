use chrono::NaiveDate;

const NUMBER_MINUTES_IN_HOUR: u16 = 60;

#[derive(PartialEq)]
pub enum Type {
    Day,
    App(String),
}

#[derive(Default)]
pub struct Time {
    hour: u16,
    min: u16,
}

impl Time {
    pub fn new(mins: u16) -> Time {
        Time {
            hour: mins / NUMBER_MINUTES_IN_HOUR,
            min: mins % NUMBER_MINUTES_IN_HOUR,
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.hour == 0 {
            return write!(f, "{}m", self.min);
        } else if self.min == 0 {
            return write!(f, "{}h", self.hour);
        } else if self.min < 10 {
            return write!(f, "{}h0{}", self.hour, self.min);
        }
        write!(f, "{}h{}", self.hour, self.min)
    }
}

pub struct TimeApp {
    pub name: String,
    pub time: Time,
    pub date: NaiveDate,
    pub min_total: u16,
}

impl TimeApp {
    pub fn new(name: String, date: NaiveDate, mins: u16) -> TimeApp {
        TimeApp {
            name,
            time: Time::new(mins),
            date,
            min_total: mins,
        }
    }
}

pub struct ListTimeApp {
    values: Vec<TimeApp>,
    type_data: Type,
    date: NaiveDate,
}

impl ListTimeApp {
    pub fn new(type_data: Type, mut values: Vec<TimeApp>, date: NaiveDate) -> ListTimeApp {
        if type_data == Type::Day {
            values.sort_unstable_by_key(|item| (item.min_total));
            values.reverse();
        }
        ListTimeApp {
            type_data,
            values,
            date,
        }
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

                write!(f, "{}", output)
            }
            Type::App(name) => {
                let mut output = String::new();
                output.push_str(&format!("\tTime for {} :\n", name));
                for v in &self.values {
                    output.push_str(&format!("{} : {}\n", v.date, v.time));
                }

                output.push_str(&format!(
                    "\n\tStats of time for {} :\n{}\n",
                    name,
                    Stat::new(&self.values)
                ));

                write!(f, "{}", output)
            }
        }
    }
}

#[derive(Default)]
pub struct Stat {
    pub max: Time,
    pub min: Time,
    pub sum: Time,
    pub mean: Time,
}

impl Stat {
    pub fn new(values: &[TimeApp]) -> Stat {
        if values.is_empty() {
            return Stat::default();
            // return Stat { max : Time::new(0), min : Time::new(0), sum : Time::new(0), mean : Time::new(0) };
        }

        let mut sum = values[0].min_total;
        let mut min = values[0].min_total;
        let mut max = values[0].min_total;

        for v in values.iter().skip(1) {
            sum += v.min_total;

            if min > v.min_total {
                min = v.min_total;
            }

            if max < v.min_total {
                max = v.min_total;
            }
        }

        Stat {
            max: Time::new(max),
            min: Time::new(min),
            sum: Time::new(sum),
            mean: Time::new(sum / values.len() as u16),
        }
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Max : {}\nMin : {}\nSum : {}\nMean: {}",
            self.max, self.min, self.sum, self.mean
        )
    }
}
