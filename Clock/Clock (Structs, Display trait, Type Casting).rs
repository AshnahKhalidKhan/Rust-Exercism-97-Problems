use std::fmt;

fn main() {
    let mut c = setClock(22, 33);
    println!("{}", c);
    setHours(&mut c, 2);
    println!("{}", c);
    addMinutes(&mut c, 60*13);
    println!("{}", c);
    subtractMinutes(&mut c, 13);
    println!("{}", c);
    subtractMinutes(&mut c, 120);
    println!("{}", c);
    subtractMinutes(&mut c, 60*12);
    println!("{}", c);
    let mut d = setClock(4, 60);
    println!("d = {}", d);
    println!("c = {}", c);
    println!("c = d: {}", c.sameClock(&d));
    println!("d = c: {}", d.sameClock(&c));
    println!("d = d: {}", d.sameClock(&d));
    println!("c = c: {}", c.sameClock(&c));
    d.oneHourPassed();
    println!("d oneHourPassed: {}", d);
}

struct Clock
{
    hours: u32,
    minutes: u32
}

fn setClock(hours: u32, minutes: u32) -> Clock
{
    let mut newClock = Clock
    {
        hours: 0,
        minutes:0
    };
    setTime(&mut newClock, hours, minutes);
    newClock
}

fn setTime(clock: &mut Clock, hours: u32, minutes: u32)
{
    setHours(clock, hours);
    setMinutes(clock, minutes);
}

fn setHours(clock: &mut Clock, hours: u32)
{
    addMinutes(clock, hours * 60)
}

fn setMinutes(clock: &mut Clock, minutes: u32)
{
    addMinutes(clock, minutes)
}

fn addMinutes(clock: &mut Clock, minutes: u32)
{
    let currentMinutes = clock.minutes;
    let mut newMinutes = currentMinutes + minutes;
    if newMinutes < 60
    {
        clock.minutes = newMinutes;
    }
    else
    {
        let currentHours = clock.hours;
        let mut newHours = newMinutes / 60;
        newHours = currentHours + newHours;
        newHours = newHours % 24;
        clock.hours = newHours;
        newMinutes = newMinutes % 60;
        clock.minutes = newMinutes;
    }
}

fn subtractMinutes(clock: &mut Clock, minutes: u32)
{
    let mut currentMinutes = clock.minutes as i32;
    let mut newMinutes = currentMinutes - minutes as i32;
    if newMinutes > 0
    {
        clock.minutes = newMinutes as u32;
    }
    else
    {
        let mut currentHours = clock.hours as i32;
        while newMinutes < 0
        {
            let mut newHours = currentHours - 1;
            if newHours < 0
            {
                newHours = newHours + 24;
            }
            newHours = newHours % 24;
            currentHours = newHours;
            newMinutes = newMinutes + 60;
            currentMinutes = newMinutes;
        }
        clock.hours = currentHours as u32;
        clock.minutes = currentMinutes as u32;
    }
}
/*
    hours: 13, minutes: 20
    subtract: 120 minutes
    clock.minutes = clock.minutes - 120;     20 - 120 = -100
    while clock.minutes < 0
    {
        clock.hours = clock.hours - 1;      13 - 1 = 12          12 - 1 = 11        
        clock.minutes = clock.minutes + 60;     -100 + 60 = - 40       -40 + 60 = 20       
    }
*/

impl fmt::Display for Clock
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Clock = {}:{}", self.hours, self.minutes)
    }
}

impl Clock
{
    fn sameClock(&self, otherClock: &Clock) -> bool
    {
        if self.hours == otherClock.hours && self.minutes == otherClock.minutes
        {
            true
        }
        else
        {
            false
        }
    }
    
    fn oneHourPassed(&mut self)
    {
        setHours(self, 1);
    }
}
