use std::fmt::{Display, Formatter};

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_DAY: i32 = HOURS_PER_DAY * MINUTES_PER_HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock2(i32, i32);

impl Clock2 {
    pub fn new(hours: i32, minutes: i32) -> Self {
        dbg!(hours, minutes);
        let mut clock_hours = match hours {
            h if h < -48 => 24 + hours % 24,
            h if h < -24 => 24 + h / 24,
            h if h < 0 => 24 + h,
            _ => hours % 24,
        };
        clock_hours += minutes / 60;
        let clock_minutes = match minutes {
            m if m < -60 => 60 + minutes % 60,
            m if m < 0 => 60 + m,
            _ => minutes % 60,
        };
        Clock2(clock_hours, clock_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock2::new(0, self.1 + minutes)
    }
}

impl Display for Clock2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0 % 24, self.1 % 60)
    }
}

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock((hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock((self.0 + minutes).rem_euclid(MINUTES_PER_DAY))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0 / MINUTES_PER_HOUR,
            self.0 % MINUTES_PER_HOUR
        )
    }
}
