/*
  BEFORE STARTING:
  Add rand to dependencies in Cargo.toml
  With this line: rand = "0.8.5"
*/

use rand::Rng;

pub fn private_key(p: u64) -> u64
{
    //unimplemented!("Pick a private key greater than 1 and less than {p}")
    let x = rand::thread_rng().gen_range(2..=p);
    println!("p: {}  x: {}", p,x);
    x
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64
{
    //unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    println!("g: {}  a: {}  p: {}      ({}*{})%{}={}",g,p,a,g,p,a,(g*a) % p);
    (g*a) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    println!("b_pub: {}  a: {}  p: {}      ({}*{})%{}={}",b_pub,p,a,b_pub,p,a,(b_pub*a) % p);
    (b_pub*a) % p
}
