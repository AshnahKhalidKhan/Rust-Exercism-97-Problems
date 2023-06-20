pub fn square_of_sum(n: u32) -> u32 {
    //unimplemented!("square of sum of 1...{n}")
    let mut number: u32 = 1;
    let mut sum: u32 = 0;
    while number <= n
    {
        sum = sum + number;
        number = number + 1;
    }
    let mut squareOfSum: u32 = sum*sum;
    squareOfSum
}

pub fn sum_of_squares(n: u32) -> u32 {
    //unimplemented!("sum of squares of 1...{n}")
    let mut number: u32 = 1;
    let mut sumOfSquares: u32 = 0;
    while number <= n
    {
        sum = sum + (number*number);
        number = number + 1;
    }
    squareOfSum
}

pub fn difference(n: u32) -> u32 {
    //unimplemented!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
