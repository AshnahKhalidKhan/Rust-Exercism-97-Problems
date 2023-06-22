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

    if a % 2 == 0
    {
        let mut n: u128 = 0;
        let mut answer: u128 = g as u128;
        while n < a as u128
        {
            answer =  (answer * answer) % p as u128;
            println!("{}", answer);
            n = n + 1;
        }
        return answer as u64;
    }
    else
    {
        
    }
    let mut n: u128 = 0;
    let mut answer: u128 = 1;
    while n < a as u128
    {
        answer =  (answer * g as u128) % p as u128;
        println!("{}", answer);
        n = n + 1;
    }
    answer as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    //b_pub.pow(a as u32) % p
    let mut n: u128 = 0;
    let mut answer: u128 = 1;
    while n < a as u128
    {
        answer =  (answer * b_pub as u128) % p as u128;
        println!("{}", answer);
        n = n + 1;
    }
    answer as u64
}
