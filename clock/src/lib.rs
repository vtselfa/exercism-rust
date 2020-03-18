use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        Clock {
            hours: 0, 
            minutes: 0,
        }.add_minutes(h * 60 + m)
    }

    pub fn add_minutes(&self, mut m: i32) -> Self {
        if m < 0 {
            let day_mins = 60 * 24;
            m = m % day_mins + day_mins;
        }
        Clock {
            hours: (self.hours + (self.minutes + m) / 60) % 24, 
            minutes: (self.minutes + m) % 60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
