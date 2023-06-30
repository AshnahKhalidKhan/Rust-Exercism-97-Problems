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

    if from_base < 2
    {
        return Err(Error::InvalidInputBase);
    }
    else if to_base < 2
    {
        return Err(Error::InvalidOutputBase);
    }
    else
    {
        let mut fromBaseToDecimal: Vec<u32> = vec![0; number.len()];
        for n in 0..number.len()
        {
            if number[n] >= from_base
            {
                return return Err(Error::InvalidDigit(number[n]));
            }
            fromBaseToDecimal[n] = number[n]*(from_base.pow((number.len() - 1 - n) as u32));
        }

        let mut sum: u32 = fromBaseToDecimal.iter().sum();

        if sum == 0
        {
            return Ok(vec![0]);
        }
            let mut decimalToToBase: Vec<u32> = Vec::new();
            while sum > 0
            {
                println!("{} % {} = {}", sum, to_base, (sum % to_base));
                decimalToToBase.push(sum % to_base);
                sum = sum / to_base;
            }
            decimalToToBase.reverse();
            return Ok(decimalToToBase);
    }
}
