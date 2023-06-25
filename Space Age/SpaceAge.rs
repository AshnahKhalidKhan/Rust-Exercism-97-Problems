// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration
{
    seconds: f64
}

impl From<u64> for Duration
{
    fn from(s: u64) -> Self
    {
        //unimplemented!("s, measured in seconds: {s}")
        Self
        {
            seconds: s as f64
        }
    }
}

macro_rules! spaceAge
{
    (Mercury, $s:expr)  => ($s/(0.2408467*31557600.0));
    (Venus, $s:expr)  => ($s/(0.61519726*31557600.0));
    (Earth, $s:expr)  => ($s/31557600.0);
    (Mars, $s:expr)  => ($s/(1.8808158*31557600.0));
    (Jupiter, $s:expr)  => ($s/(11.862615*31557600.0));
    (Saturn, $s:expr)  => ($s/(29.447498*31557600.0));
    (Uranus, $s:expr)  => ($s/(84.016846*31557600.0));
    (Neptune, $s:expr)  => ($s/(164.79132*31557600.0));
}

pub trait Planet
{
    fn years_during(d: &Duration) -> f64;
    // {
    //     unimplemented!("convert a duration ({d:?}) to the number of years on this planet for that duration");
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
        spaceAge!(Mercury, d.seconds)
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
