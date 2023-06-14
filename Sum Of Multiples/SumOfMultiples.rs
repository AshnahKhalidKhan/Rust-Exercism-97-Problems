pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //unimplemented!("Sum the multiples of all of {factors:?} which are less than {limit}")
    
    let mut multiples: Vec<u32> = Vec::new();
    println!("Limit: {}", limit);
    println!("Factors: {:?}", factors);

    for index in 0..factors.len()
    {
        if factors[index] == 0
        {
            multiples.push(factors[index]);
        }
        else
        {
            let mut finalMultiplier: u32 = limit / factors[index];
            println!("finalMultiplier: {}", finalMultiplier);
            // let mut multiplier: u32 = 0;
            for multiplier in 1..=finalMultiplier
            {
                if multiples.contains(&(factors[index] * multiplier)) == false && factors[index] * multiplier < limit
                {
                    println!("{} x {} = {}", factors[index], multiplier, factors[index] * multiplier);
                    multiples.push(factors[index] * multiplier);
                }
            }
        }
        // let mut multiplier: u32 = 1;
        // while (factors[index] * multiplier) < limit
        // {
        //     println!("{} x {} = {}", factors[index], multiplier, factors[index] * multiplier);
        //     // if multiples.contains(&(factors[index] * multiplier)) == false
        //     // {
        //     //     multiples.push(factors[index] * multiplier);
        //     // }
        //     multiplier = multiplier + 1;
        // }
        // while multiplier <= limit
        // {
        //     multiplier = multiplier + factors[index];
        //     println!("{}", multiplier);
        //     if multiples.contains(&multiplier) == false
        //     {
        //         multiples.push(multiplier);
        //     }
        // }
    }
    println!("{:?}", multiples);

    multiples.iter().sum()
}
