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
    println!("g: {}  a: {}  p: {}      ({}*{})%{}={}",g,a,p,g,a,p,(g*a) % p);
    //((g % p)*(a % p)) % p
    g.pow(a as u32) % p
    //((g % p)*a) % p
    // ((((1/g).pow(a as u32)) as f64).powi(-1) % (p as f64)) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    println!("b_pub: {}  a: {}  p: {}      ({}*{})%{}={}",b_pub,a,p,b_pub,a,p,(b_pub*a) % p);
    //((b_pub % p)*(a % p)) % p
    b_pub.pow(a as u32) % p
    //((b_pub % p)*a) % p
    //((((1/b_pub).pow(a as u32)) as f64).powi(-1) % (p as f64)) as u64
}
