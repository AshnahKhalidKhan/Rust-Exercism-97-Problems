fn main()
{
    println!("{:?}", factors(23780)); //Replace 23780 with whatever number you want to factorize
}

pub fn factors(n: u64) -> Vec<u64>
{
    //unimplemented!("This should calculate the prime factors of {n}")
    // This code works but takes too long
    let mut PrimeFactors: Vec<u64> = Vec::new();
    let mut PrimeNumber: u64 = 2;
    let mut Remainder: u64 = n;

    while Remainder > 1
    {
        //Check if number is a prime
        let mut factorsOfCurrentPrimeNumber: u64 = 2;
        let mut isPrime: bool = true;
        while factorsOfCurrentPrimeNumber < PrimeNumber
        {
            if PrimeNumber % factorsOfCurrentPrimeNumber == 0
            {
                isPrime = false;
                break;
            }
            factorsOfCurrentPrimeNumber = factorsOfCurrentPrimeNumber + 1;
        }
        if isPrime == true
        {
            while Remainder % PrimeNumber == 0
            {
                PrimeFactors.push(PrimeNumber);
                Remainder = Remainder / PrimeNumber;
            }
        }
        PrimeNumber = PrimeNumber + 1;
    }
    PrimeFactors
}
