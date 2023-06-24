pub struct Clock
{
    hours: i32,
    minutes: i32
}

impl Clock
{
    pub fn new(hours: i32, minutes: i32) -> Self
    {
        //unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Self
        {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self
    {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}

use std::fmt;

impl fmt::Display for Clock
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut h: String = self.hours.to_string();
        let mut m: String = self.minutes.to_string();
        if self.hours < 10
        {
            h = "0".to_owned() + &h;
        }
        if self.minutes < 10
        {
            m = "0".to_owned() + &m;
        }
        write!(f, "{}:{}", h, m)
    }
}

impl fmt::Debug for Clock
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Clock")
         .field("hours", &self.hours)
         .field("minutes", &self.minutes)
         .finish()
    }
}
