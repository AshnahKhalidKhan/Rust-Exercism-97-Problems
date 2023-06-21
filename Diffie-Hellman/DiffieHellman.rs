/*
  BEFORE STARTING:
  Add rand to dependencies in Cargo.toml
  With this line: rand = "0.8.5"
*/

use rand::Rng;

pub fn private_key(p: u64) -> u64
{
    //unimplemented!("Pick a private key greater than 1 and less than {p}")
    let x = rand::thread_rng().gen_range(2..p);
    println!("p: {}  x: {}", p,x);
    x
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64
{
    //unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    //println!("g: {}  a: {}  p: {}      ({}*{})%{}={}",g,a,p,g,a,p,(g*a) % p);
    //((g % p)*(a % p)) % p
    // g.pow(a as u32) % p
    //((g % p)*a) % p
    // ((((1/g).pow(a as u32)) as f64).powi(-1) % (p as f64)) as u64
    // let mut n = 0;
    // let mut answer: u128 = 1 as u128;
    // while n < a
    // {
    //     answer = (answer * g  as u128) % p  as u128;
    //     n = n + 1;
    // }
    // //(answer % p  as u128) as u64
    // answer as u64
    let mut n: u32 = 0;
    let mut answer: u64 = 1;
    while n < a as u32
    {
        // println!("gwalay: {}", answer);
        //answer =  (answer * answer) % p;
        answer =  (answer * g) % p;
        println!("{}", answer);
        n = n + 1;
    }
    answer % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    //println!("b_pub: {}  a: {}  p: {}      ({}*{})%{}={}",b_pub,a,p,b_pub,a,p,(b_pub*a) % p);
    //((b_pub % p)*(a % p)) % p
    //b_pub.pow(a as u32) % p
    //((b_pub % p)*a) % p
    //((((1/b_pub).pow(a as u32)) as f64).powi(-1) % (p as f64)) as u64
    // let mut n = 0;
    // let mut answer = b_pub;
    // while n < a
    // {
    //     answer = answer * b_pub;
    //     answer = answer % p;
    //     n = n + 1;
    // }
    // answer % p
    // let mut n = 0;
    // let mut answer: u128 = 1 as u128;
    // while n < a
    // {
    //     answer = (answer * b_pub  as u128) % p  as u128;
    //     n = n + 1;
    // }
    // // (answer % p  as u128) as u64
    // answer as u64
    let mut n: u32 = 0;
    let mut answer: u64 = 1;
    while n < a as u32
    {
        // println!("bpub walau: {}", answer);
        //answer =  (answer * answer) % p;
        answer =  (answer * b_pub) % p;
        println!("{}", answer);
        n = n + 1;
    }
    answer % p
}
