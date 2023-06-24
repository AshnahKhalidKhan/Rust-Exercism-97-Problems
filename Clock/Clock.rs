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
        let mut h: i32 = hours;
        let mut m: i32 = minutes;
        println!("{}:{}", h, m);
        while m < 0
        {
            m = m + 60;
            h = h - 1;
        }
        while m > 59
        {
            m = m - 60;
            h = h + 1;
        }
        m = ((m % 60) + 60) % 60;
        h = ((h % 24) + 24) % 24;
        println!("{}:{}", h, m);
        
        Self
        {
            hours: h,
            minutes: m
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self
    {
        //unimplemented!("Add {minutes} minutes to existing Clock time");
        Clock::new(self.hours, self.minutes + minutes)
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

impl PartialEq for Clock
{
    fn eq(&self, other: &Self) -> bool
    {
        ((self.hours % 24) == (other.hours % 24)) && ((self.minutes % 60) == (other.minutes % 60))
    }
}
