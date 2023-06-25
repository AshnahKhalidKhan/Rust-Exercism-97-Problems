// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration;
// {
//     seconds: u64
// }

impl From<u64> for Duration
{
    fn from(s: u64) -> Self
    {
        //unimplemented!("s, measured in seconds: {s}")
        // Self
        // {
        //     seconds: s
        // }
        s
    }
}

macro_rules! spaceAge
{
    ($planet: ident, $s:expr) =>
    {
        match $planet
        {
            Mercury => $s as f64,
            _ => $s as f64
        }
    }
}

pub trait Planet
{
    fn years_during(d: &Duration) -> f64;
    // {
    //     //unimplemented!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    //     Planet::years_during(d)
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
        spaceAge!(Mercury, d)
    }
}

impl Planet for Venus
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Venus, d)
    }
}

impl Planet for Earth
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Earth, d)
    }
}

impl Planet for Mars
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Mars, d)
    }
}

impl Planet for Jupiter
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Jupiter, d)
    }
}

impl Planet for Saturn
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Saturn, d)
    }
}

impl Planet for Uranus
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Uranus, d)
    }
}

impl Planet for Neptune
{
    fn years_during(d: &Duration) -> f64
    {
        spaceAge!(Neptune, d)
    }
}
