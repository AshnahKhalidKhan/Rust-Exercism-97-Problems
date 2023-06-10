
fn main()
{
    println!("{:?}", factors(93819012551)); //Replace 23780 with whatever number you want to factorize
}

pub fn factors(n: u64) -> Vec<u64>
{
    //unimplemented!("This should calculate the prime factors of {n}")
    let mut PrimeFactors: Vec<u64> = Vec::new();
    let mut PrimeNumber: u64 = 2;
    let mut Quotient: u64 = n;
    while Quotient > 1
    {
        while Quotient % PrimeNumber == 0
        {
            PrimeFactors.push(PrimeNumber);
            Quotient = Quotient / PrimeNumber;
        }
        PrimeNumber = PrimeNumber + 1;
    }
    PrimeFactors
}
