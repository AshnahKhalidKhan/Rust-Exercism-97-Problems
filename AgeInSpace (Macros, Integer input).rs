/******************************************************************************
Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
use std::env;

macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

macro_rules! ageOnMercury
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(0.2408467*31557600.0));
                    };
}

macro_rules! ageOnVenus
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(0.61519726*31557600.0));
                    };
}

macro_rules! ageOnMercury
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(0.2408467*31557600.0));
                    };
}

macro_rules! ageOnEarth
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(1.0*31557600.0));
                    };
}

macro_rules! ageOnMars
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(1.8808158*31557600.0));
                    };
}

macro_rules! ageOnJupiter
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(11.862615*31557600.0));
                    };
}

macro_rules! ageOnSaturn
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(29.447498*31557600.0));
                    };
}

macro_rules! ageOnUranus
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(84.016846*31557600.0));
                    };
}

macro_rules! ageOnNeptune
{
    ($seconds: expr) =>
                    {
                        println!("{}", $seconds/(164.79132*31557600.0));
                    };
}

fn main()
{
    let vectorWithCMDInputAndThatExtraString: Vec<String> = env::args().collect();
    println!("Raw CMD input: {:?}", vectorWithCMDInputAndThatExtraString);
    
    let seconds: f32 = match vectorWithCMDInputAndThatExtraString[1].parse()
    {
        Ok(i) =>
                {
                    i
                },
        Err(_) =>
                {
                    0.0
                }
    };
    println!("Seconds given: {}", seconds);
    ageOnMercury!(seconds);
    ageOnVenus!(seconds);
    ageOnEarth!(seconds);
    ageOnMars!(seconds);
    ageOnJupiter!(seconds);
    ageOnSaturn!(seconds);
    ageOnUranus!(seconds);
    ageOnNeptune!(seconds);
    
    say_hello!();
}

