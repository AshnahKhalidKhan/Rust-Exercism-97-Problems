pub fn square(s: u32) -> u64
{
    //unimplemented!("grains of rice on square {s}");
    if s > 64 || s < 1
    {
        panic!("Square must be between 1 and 64");
    }
    let grain: u64 = 2;
    grain.pow(s - 1)
}

pub fn total() -> u64
{
    //unimplemented!();
    let mut totalFortune: u64 = 0;
    for s in 1..=64
    {
        totalFortune = totalFortune + square(s);
    }
    totalFortune
}
