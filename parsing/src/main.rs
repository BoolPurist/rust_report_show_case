use std::fmt;
use std::str;

#[derive(Debug)]
struct TimeInTime {
    total_secs: u64,
}

impl TimeInTime {
    pub fn new(total_secs: u64) -> Self {
        TimeInTime { total_secs }
    }
    pub fn with_ms(mins: u64, secs: u64) -> Self {
        TimeInTime {
            total_secs: secs + (mins * 60),
        }
    }

    pub fn with_hms(hours: u64, mins: u64, secs: u64) -> Self {
        TimeInTime {
            total_secs: secs + (mins * 60) + (hours * 60 * 60),
        }
    }

    fn get_secs(&self) -> u64 {
        self.total_secs % 60
    }

    fn get_minutes(&self) -> u64 {
        (self.total_secs / 60) % 60
    }

    fn get_hours(&self) -> u64 {
        self.total_secs / (60 * 60)
    }

    pub fn get_secs_minutes_hours(&self) -> (u64, u64, u64) {
        (self.get_hours(), self.get_minutes(), self.get_secs())
    }
}

#[derive(Debug)]
enum TimeParseError {
    InvalidPositiveNumberFormat,
    MoreThan3Units,
}

const HOUR_MIN_SEC_INPUT: usize = 3;
const MIN_SEC_INPUT: usize = 2;
const SEC_INPUT: usize = 1;

impl str::FromStr for TimeInTime {
    type Err = TimeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let separator: Vec<&str> = s.split(':').collect();

        return match separator.len() {
            len if len == HOUR_MIN_SEC_INPUT => {
                let secs: u64 = separator[2].parse()?;
                let mins: u64 = separator[1].parse()?;
                let hours: u64 = separator[0].parse()?;

                Ok(TimeInTime::with_hms(hours, mins, secs))
            }
            len if len == MIN_SEC_INPUT => {
                let secs: u64 = separator[0].parse()?;
                let mins: u64 = separator[1].parse()?;

                Ok(TimeInTime::with_ms(mins, secs))
            }
            len if len == SEC_INPUT => {
                let secs: u64 = separator[0].parse()?;

                Ok(TimeInTime::new(secs))
            }
            _ => Err(TimeParseError::MoreThan3Units),
        };
    }
}

impl From<std::num::ParseIntError> for TimeParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        TimeParseError::InvalidPositiveNumberFormat
    }
}
impl fmt::Display for TimeInTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_hms = self.get_secs_minutes_hours();
        writeln!(f, "{}:{}:{}", time_hms.0, time_hms.1, time_hms.2)
    }
}
fn main() {
    let data: TimeInTime = "12:56:12".parse().unwrap();
    println!("{}", data);
}
