use std::cell::Cell;
use std::fmt::{Display,Formatter,Result};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: Cell<i32>,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes: Cell::new(minutes) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: Cell::new(self.minutes.get() + minutes),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.hours, self.minutes.get())
    }
}
