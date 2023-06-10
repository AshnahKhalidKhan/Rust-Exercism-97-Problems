/******************************************************************************
Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
fn main() {
    println!("# print_fib");
    print_fib(20);

    println!("\n# make_fib");
    println!("{:?}", make_fib(20));

    println!("\n# iterator");
    for (i, x) in Fib::new().take(20).enumerate() {
        println!("{}: {}", i, x);
    }
    
    
    
    let fib = Fibonnaci::new();
    let sequence: Vec<u32> = fib.take(20).collect();
    println!("{:?}", sequence);
    
}

struct Fibonnaci
{
    num1: u32,
    num2: u32
}

impl Fibonnaci
{
    fn new() -> Self
    {
        Fibonnaci
        {
            num1: 0,
            num2: 1
        }
    }
}

use std::iter::Iterator;

impl Iterator for Fibonnaci
{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>
    {
        let num3 = self.num1;
        self.num1 = self.num2;
        self.num2 = self.num2 + num3;
        Some(num3)
    }
}

fn print_fib(n: usize) {
    let mut x = (1, 1);
    for i in 0..n {
        println!("{}: {}", i, x.0);
        x = (x.1, x.0 + x.1)
    }
}

fn make_fib(n: usize) -> Vec<i32> {
    let mut x = vec![1, 1];
    for i in 2..n {
        let next_x = x[i - 1] + x[i - 2];
        x.push(next_x)
    }
    x
}

struct Fib {
    x: (usize, usize),
}

impl Fib {
    fn new() -> Fib {
        Fib { x: (0, 1) }
    }
}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.0)
    }
}