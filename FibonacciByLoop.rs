use std::io;

#[allow(unused_assignments)]
fn main() {
    //Reading from console
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Excuse me, trying to read a number here!");
        
    //Parsing n into an unsigned integer
    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    println!("Find the {}th Fibonacci number (starting from index 0):", n);
    
    //It's fibonacci time!
    let mut x = 0;
    let mut x_minus_1 = 1;
    let mut y = 1;
    let mut count = 0;
    loop {
        /* 
           x-1  x   y
            0 + 1 = 1
            y + 1 = 2
            y + 1 = 3
            y + 2 = 5
        */
        if count == 0 || count == 1 {
            println!("Fibonacci term # {}: {}", count, count);
            count += 1;
        }
        else if count == n {
            break;
        }
        else {
            y = x + x_minus_1;
            x = x_minus_1;
            x_minus_1 = y;
            println!("Fibonacci term # {}: {}", count, y);
            count += 1;
        }
    };
}