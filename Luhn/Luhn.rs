/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool
{
    //unimplemented!("Is the Luhn checksum for {code} valid?");
    
    /*
        Remove whitespaces
        Remove non-digit characters
        Check if length < 2
        Reverse array
        Double every second digit
        Minus 9 from all doubled digits greater than 9
        Sum all doubled resultant digits
        Check if sum % 10 == 0
    */

    println!("Original: {}", code);
    
    let mut onlyDigits: Vec<i128> = Vec::new();
    for c in code.chars()
    {
        if c.is_ascii_digit() == true
        {
            onlyDigits.push(c.to_digit(10).unwrap() as i128);
        }
    }

    println!("Only digits: {:?}", onlyDigits);
        if code.len() < 2
    {
        false
    }
    else
    {
        onlyDigits.reverse();

        println!("Only digits reversed: {:?}", onlyDigits);
        
        for i in 0..onlyDigits.len()
        {
            onlyDigits[i] = match (i % 2 == 1, onlyDigits[i]*2)
            {
                (true, x) if x > 9 => x - 9,
                (true, x) => x,
                _ => continue
            };
        }
    
        println!("Only digits after doubling & -9s: {:?}", onlyDigits);

        let sum: i128 = onlyDigits.iter().sum();

        println!("sum: {}", sum);
        
        match sum % 10
        {
            0 => true,
            _ => false
        }
    }
}
