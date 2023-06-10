
fn main()
{
    println!("{:?}", factors(93819012551)); //Replace 23780 with whatever number you want to factorize
}

pub fn factors(n: u64) -> Vec<u64>
{
    //unimplemented!("This should calculate the prime factors of {n}")

    let mut AllPrimeNumbers: Vec<u64> = Vec::new();
    let mut PrimeNumber: u64 = 2;
    while PrimeNumber <= n/2
    {
        //Check if number is a prime
        let mut factorsOfCurrentPrimeNumber: u64 = 2;
        let mut isPrime: bool = true;
        while factorsOfCurrentPrimeNumber < PrimeNumber/2
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
            println!("{:?}", AllPrimeNumbers);
            AllPrimeNumbers.push(PrimeNumber);
        }
        PrimeNumber = PrimeNumber + 1;
    }

    let mut PrimeFactors: Vec<u64> = Vec::new();
    let mut Remainder: u64 = n;
    while Remainder > 1
    {
        while Remainder % AllPrimeNumbers[0] == 0
        {
            println!("{}", AllPrimeNumbers[0]);
            PrimeFactors.push(AllPrimeNumbers[0]);
            Remainder = Remainder / AllPrimeNumbers[0];
        }
        AllPrimeNumbers.remove(0);
    }
    PrimeFactors
}
