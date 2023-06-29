#[derive(Debug, PartialEq, Eq)]
pub enum Error
{
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///


// pub enum Error
// {
//     InvalidInputBase,
//     InvalidOutputBase,
//     InvalidDigit(u32),
// }

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error>
{
    //unimplemented!("Convert {number:?} from base {from_base} to base {to_base}")

    if from_base < 2
    {
        return Err(Error::InvalidInputBase);
    }
    else if to_base < 2
    {
        return Err(Error::InvalidOutputBase);
    }
    else if number == [] || number.len() == 0
    {
        return Ok(vec![0]);
    }
    else
    {
        println!("number: {:?}", number);
        println!("from_base: {}", from_base);
        println!("to_base: {}", to_base);

        let mut givenBaseToDecimal: Vec<u32> = vec![0; number.len()];
        for n in 0..number.len()
        {
            if number[n] >= from_base
            {
                return return Err(Error::InvalidDigit(number[n]));
            }
            givenBaseToDecimal[n] = number[n]*(from_base.pow((number.len() - 1 - n) as u32));
        }

        let mut sum: u32 = givenBaseToDecimal.iter().sum();

        println!("givenBaseToDecimal: {:?}", givenBaseToDecimal);
        println!("sum: {}", sum);

        if to_base == 10
        {
            let mut s: String = sum.to_string();
            return Ok(s.chars().map(|d| d.to_digit(10).unwrap()).collect());
        }
        else
        {
            let mut digits: String = String::new();
            // while sum > to_base
            // {
            //     digits = (sum % to_base).to_string() + &digits;
            //     sum = sum / to_base;
            // }
        }
        return Ok(number.to_vec());
    }
}
