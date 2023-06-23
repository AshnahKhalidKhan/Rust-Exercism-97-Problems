/*
  BEFORE STARTING:
  Add rand to dependencies in Cargo.toml
  With this line: rand = "0.8.5"
*/

use rand::Rng;

pub fn private_key(p: u64) -> u64
{
    //unimplemented!("Pick a private key greater than 1 and less than {p}")
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64
{
    //unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    
    println!("g^a % p = {}^{} % {}", g, a, p);
  
    /*
        Direct solution would be: g.pow(a as u32) % p
        Instead, first we find the nearest power of 2 to the given 'a'.
        The first time, we multiply 'g' with 1 and find it's mod with 'p'; this is 'answer'.
        While we have not reached that power of 2, we square 'answer' and double our counter until we reach the nearest power of 2 to the given 'a'.
        If 'a' was not a power of 2, the counter will be less than 'a' after this step.
        While the counter is less than 'a', we multiply 'g' 'a' minus 'n' number of times with itself, modding it with 'p' each time and increasing the counter by 1; this is 'other answer'.
        If 'a' were a power of 2, 'other answer' remains 1.
        Finally, we multiple 'answer' & 'other answer', mod it with 'p', and that's our final result.
    */
    
    let mut Two: u64 = 2;
    let closestPowerOfTwoToGivenA: u64 = Two.pow((a as f64).log2().floor() as u32) as u64;
    let mut answer: u64 = 1;
    let mut n: u64 = 0;
    while n < closestPowerOfTwoToGivenA
    {
        if n == 0
        {
            answer = (answer * g) % p;
            n = n + 1;
        }
        else
        {
            answer = (answer * answer) % p;
            n = n + n;
        }
    }
    let mut theOtherNumber: u64 = 1;
    while n < a && closestPowerOfTwoToGivenA != a
    {
        theOtherNumber = (theOtherNumber * g) % p;
        n = n + 1;
    }
    answer = (answer * theOtherNumber) % p;
    answer
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    
    println!("b_pub^a % p = {}^{} % {}", b_pub, a, p);
  
    //Direct solution would be: b_pub.pow(a as u32) % p
  
    public_key(p, b_pub, a)
}
