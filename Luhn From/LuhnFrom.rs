pub struct Luhn<T>
{
    code: T
}

impl<T> Luhn<T>
{
    pub fn is_valid(&self) -> bool
    {
        unimplemented!("Determine if the current Luhn struct contains a valid credit card number.");
    
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
    
        //println!("Original: {:?}", self.code);

        // let code1: String = String::from(self.code);
        
        // let mut onlyDigits: Vec<i128> = Vec::new();
        // for c in self.code.chars()
        // {
        //     match (c.is_ascii_digit(), c.is_whitespace())
        //     {
        //         (false, false) => return false,
        //         (true, _) => onlyDigits.push(c.to_digit(10).unwrap() as i128),
        //         (_, true) => continue,
        //     };
        // }
    
        // println!("Only digits: {:?}", onlyDigits);
        
        // if onlyDigits.len() < 2
        // {
        //     false
        // }
        // else
        // {
        //     onlyDigits.reverse();
        //     for i in 0..onlyDigits.len()
        //     {
        //         onlyDigits[i] = match (i % 2 == 1, onlyDigits[i]*2)
        //         {
        //             (true, x) if x > 9 => x - 9,
        //             (true, x) => x,
        //             _ => continue
        //         };
        //     }
        //     let sum: i128 = onlyDigits.iter().sum();
        //     match sum % 10
        //     {
        //         0 => true,
        //         _ => false
        //     }
        // }
        //true
    }
}

impl<T> From<T> for Luhn<T> where String: From<T>
{
    fn from(input: T) -> Self
    {
        //unimplemented!("From the given input '{input}' create a new Luhn struct.");

        let x = String::from(input);
        Self
        {
            code: x
        }
    }
}
