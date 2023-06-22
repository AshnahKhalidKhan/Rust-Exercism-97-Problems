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

    if (a as f64).log2().ceil() == (a as f64).log2() && (a as f64).log2().floor() == (a as f64).log2() 
    {
        let mut answer: u64 = 1;
        let mut n: u64 = 0;
        let mut theNumberTwo: u64 = 2;
        println!("g={}, a={}, p={}", g, a, p);
        println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        while n < a
        {
            println!("n={}", n);
            if n == 0
            {
                println!("multiplying by {}", g);
                answer = (answer * g) % p;
                n = n + 1;
            }
            else
            {
                println!("squaring");
                answer = (answer * answer) % p;
                n = n + n;
            }
            //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        }
        return answer;
    }
    else
    {
        let closestPowerOfTwoToGivenA: u64 = (a as f64).log2().floor() as u64;
        println!("closest power of 2 to a{} is {}", a, closestPowerOfTwoToGivenA);
        let mut answer: u64 = 1;
        let mut n: u64 = 0;
        let mut theNumberTwo: u64 = 2;
        println!("g={}, a={}, p={}", g, a, p);
        //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        while n < closestPowerOfTwoToGivenA
        {
            println!("n={}", n);
            if theNumberTwo.pow(n as u32) % 2 == 0
            {
                println!("squaring");
                answer = (answer * answer) % p;
                n = n + n;
            }
            else
            {
                println!("multiplying by {}", g);
                answer = (answer * g) % p;
                n = n + 1;
            }
            //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        }
        while n < a
        {
            answer = (answer * g) % p;
            n = n + 1;
        }
        return answer;
    }
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64
{
    //unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
        if (a as f64).log2().ceil() == (a as f64).log2() && (a as f64).log2().floor() == (a as f64).log2() 
    {
        let mut answer: u64 = 1;
        let mut n: u64 = 0;
        let mut theNumberTwo: u64 = 2;
        //println!("g={}, a={}, p={}", g, a, p);
        //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        while n < a
        {
            println!("n={}", n);
            if n == 0
            {
                println!("multiplying by {}", b_pub);
                answer = (answer * b_pub) % p;
                n = n + 1;
            }
            else
            {
                println!("squaring");
                answer = (answer * answer) % p;
                n = n + n;
            }
            //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        }
        return answer;
    }
    else
    {
        let closestPowerOfTwoToGivenA: u64 = (a as f64).log2().floor() as u64;
        println!("closest power of 2 to a{} is {}", a, closestPowerOfTwoToGivenA);
        let mut answer: u128 = 1;
        let mut n: u64 = 0;
        let mut theNumberTwo: u64 = 2;
        println!("g={}, a={}, p={}", b_pub, a, p);
        //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        while n < closestPowerOfTwoToGivenA
        {
            println!("n={} answer={}", n, answer);
            if n == 0
            {
                println!("multiplying by {}", b_pub);
                answer = (answer * b_pub as u128) % p as u128;
                n = n + 1;
            }
            else
            {
                println!("squaring");
                answer = (answer * answer) % p as u128;
                n = n + n;
            }
            //println!("n={}, 2^n={}, answer={}", n, theNumberTwo.pow(n as u32), answer);
        }
    println!("Am out");
        // while n < a
        // {
        //     answer = (answer * b_pub) % p;
        //     n = n + 1;
        // }
        // let mut x: u128 = b_pub.pow((a-n) as u32) as u128;
        // x = x % p as u128;
        // println!("Is literally this the problem");
        // answer = answer*x;
        // println!("or this");
        let mut theOtherNumber: u128 = 1;
        while n < a
        {
            println!("n={}", n);
            
                theOtherNumber = (theOtherNumber * b_pub as u128) % p as u128;
                n = n + 1;
        }
    answer = (answer*theOtherNumber) % p as u128;
        return answer as u64;
    }
    //b_pub.pow(a as u32) % p

}
