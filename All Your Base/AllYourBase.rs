#[derive(Debug, PartialEq, Eq)]
pub enum Error
{
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error>
{
    //unimplemented!("Convert {number:?} from base {from_base} to base {to_base}")

    match (from_base < 2, to_base < 2)
    {
        (true, _) => Err(Error::InvalidInputBase),
        (_, true) => Err(Error::InvalidOutputBase),
        (_, _) =>
        {
            let mut fromBaseToDecimal: Vec<u32> = vec![0; number.len()];
            for n in 0..number.len()
            {
                if number[n] >= from_base
                {
                    return Err(Error::InvalidDigit(number[n]));
                }
                // fromBaseToDecimal = num*(from_base^n)
                fromBaseToDecimal[n] = number[n]*(from_base.pow((number.len() - 1 - n) as u32));
            }
            
            let mut sum: u32 = fromBaseToDecimal.iter().sum();
            match sum
            {
                0 => Ok(vec![0]),
                _ =>
                {
                    let mut decimalToToBase: Vec<u32> = Vec::new();
                    while sum > 0
                    {
                        decimalToToBase.push(sum % to_base);
                        sum = sum / to_base;
                    }
                    decimalToToBase.reverse();
                    Ok(decimalToToBase)
                }
            }
        }
    }
}
