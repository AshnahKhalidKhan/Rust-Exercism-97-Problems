pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32
{
    //unimplemented!("Sum the multiples of all of {factors:?} which are less than {limit}")

    println!("Limit: {}", limit);
    println!("Factors: {:?}", factors);
    
    let mut multiples: Vec<u32> = Vec::new();
    let mut finalMultiplier: u32 = 0;

    for index in 0..factors.len()
    {
        if factors[index] == 0
        {
            multiples.push(factors[index]);
        }
        else
        {
            finalMultiplier = limit / factors[index];
            
            for n in 1..=finalMultiplier
            {
                let multiple: u32 = factors[index] * n;
                if multiple < limit && multiples.contains(&multiple) == false
                {
                    multiples.push(multiple);
                }
            }
        }
    }
    
    println!("{:?}", multiples);

    multiples.iter().sum()
}
