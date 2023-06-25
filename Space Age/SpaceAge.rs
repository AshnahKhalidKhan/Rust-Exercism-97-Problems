// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration
{
    seconds: u64
}

impl From<u64> for Duration
{
    fn from(s: u64) -> Self
    {
        //unimplemented!("s, measured in seconds: {s}")
        Self
        {
            seconds: s
        }
        // s as f64
    }
}

macro_rules! spaceAge
{
    ($planet: ident; $s:expr) =>
    {
        match $planet
        {
            Mercury => $s as f64,
            Venus => $s as f64,
            Earth => $s as f64,
            Mars => $s as f64,
            Jupiter => $s as f64,
            Saturn => $s as f64,
            Uranus => $s as f64,
            Neptune => $s as f64,
            _ => $s as f64
        }
    }
}

pub trait Planet
{
    fn years_during(d: &Duration) -> f64;
    // {
    //     //unimplemented!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    //     Planet::years_during(d.seconds)
    // }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Mercury::, d.seconds)
    }
}

impl Planet for Venus
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Venus, d.seconds)
    }
}

impl Planet for Earth
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Earth, d.seconds)
    }
}

impl Planet for Mars
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Mars, d.seconds)
    }
}

impl Planet for Jupiter
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Jupiter, d.seconds)
    }
}

impl Planet for Saturn
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Saturn, d.seconds)
    }
}

impl Planet for Uranus
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Uranus, d.seconds)
    }
}

impl Planet for Neptune
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Neptune, d.seconds)
    }
}
