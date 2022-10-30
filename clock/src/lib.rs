use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    current_time: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hours = con_hours(self.current_time).to_string();

        if hours.len() == 1 {
            hours = "0".to_string() + &hours;
        }

        let mut mins = con_minutes(self.current_time).to_string();
        if mins.len() == 1 {
            mins = "0".to_string() + &mins;
        }
        write!(f, "{}:{}", hours, mins)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock {
            current_time: (hours * 60 + minutes).rem_euclid(1440),
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = (self.current_time + minutes).rem_euclid(1440);
        return Clock::new(con_hours(time), con_minutes(time));
    }

    pub fn sub_minutes(&self, minutes: i32) -> Self {
        let time = (self.current_time - minutes).rem_euclid(1440);
        return Clock::new(con_hours(time), con_minutes(time));
    }
}

fn con_hours(time: i32) -> i32 {
    return time / 60;
}
fn con_minutes(time: i32) -> i32 {
    return time.rem_euclid(60);
}
