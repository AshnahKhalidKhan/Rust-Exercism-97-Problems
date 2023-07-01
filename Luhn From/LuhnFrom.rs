pub struct Luhn<T>
{
    self.code: T
}

impl<T> Luhn<T>
{
    pub fn is_valid(&self) -> bool
    {
        //unimplemented!("Determine if the current Luhn struct contains a valid credit card number.");
    
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
    
        println!("Original: {}", self.code);
        
        let mut onlyDigits: Vec<i128> = Vec::new();
        for c in self.code.chars()
        {
            match (c.is_ascii_digit(), c.is_whitespace())
            {
                (false, false) => return false,
                (true, _) => onlyDigits.push(c.to_digit(10).unwrap() as i128),
                (_, true) => continue,
            };
        }
    
        println!("Only digits: {:?}", onlyDigits);
        
        if onlyDigits.len() < 2
        {
            false
        }
        else
        {
            onlyDigits.reverse();
            for i in 0..onlyDigits.len()
            {
                onlyDigits[i] = match (i % 2 == 1, onlyDigits[i]*2)
                {
                    (true, x) if x > 9 => x - 9,
                    (true, x) => x,
                    _ => continue
                };
            }
            let sum: i128 = onlyDigits.iter().sum();
            match sum % 10
            {
                0 => true,
                _ => false
            }
        }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn<T>
{
    fn from(input: T) -> Self
    {
        //unimplemented!("From the given input '{input}' create a new Luhn struct.");
        Self
        {
            self.code: input
        }
    }
}
