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
    // g.pow(a as u32) % p
    println!("g={}, a={}, p={}", g, a, p);
    
    let mut theNumberTwo: u64 = 2;
    let closestPowerOfTwoToGivenA: u64 = theNumberTwo.pow((a as f64).log2().floor() as u32) as u64;
    let mut answer: u64 = 1;
    let mut n: u64 = 0;
    let mut theNumberTwo: u64 = 2;
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
    //b_pub.pow(a as u32) % p
    println!("b_pub={}, a={}, p={}", b_pub, a, p);
    
        let mut theNumberTwo: u64 = 2;
        let closestPowerOfTwoToGivenA: u64 = theNumberTwo.pow((a as f64).log2().floor() as u32) as u64;
        let mut answer: u128 = 1;
        let mut n: u64 = 0;
        while n < closestPowerOfTwoToGivenA
        {
            if n == 0
            {
                answer = (answer * b_pub as u128) % p as u128;
                n = n + 1;
            }
            else
            {
                answer = (answer * answer) % p as u128;
                n = n + n;
            }
        }
        let mut theOtherNumber: u128 = 1;
        while n < a && closestPowerOfTwoToGivenA != a
        {
                theOtherNumber = (theOtherNumber * b_pub as u128) % p as u128;
                n = n + 1;
        }
        answer = (answer * theOtherNumber) % p as u128;
        return answer as u64;
    // }
}
