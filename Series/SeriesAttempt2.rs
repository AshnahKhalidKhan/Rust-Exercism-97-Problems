pub fn series(digits: &str, len: usize) -> Vec<String> {
    //unimplemented!("What are the series of length {len} in string {digits:?}")
    
    let digitsAsCharacters: Vec<char> = digits.chars().collect();
    let mut series: Vec<String> = Vec::new();

    if digits.len() == len
    {
        series.push(digits.to_string());
        return series;
    }
    else if len == 0
    {
        for i in 0..=digits.len()
        {
            series.push(String::new());
        }
        return series;
    }
    else if digits.len() < len
    {
        return series;
    }
    else if digits.len() > len
    {
        let mut digit: usize = 0;
        while digit < digits.len()
        {
            let mut seriesElement: String = String::new();
            let mut digitsSet: usize = 0;
            while seriesElement.len() < len && (digit + digitsSet) < digits.len()
            {
                seriesElement = seriesElement + &digitsAsCharacters[digit + digitsSet].to_string();
                digitsSet = digitsSet + 1;
            }
            println!("seriesElement: {}", seriesElement);
            if seriesElement.len() == len
            {
                series.push(seriesElement);
            }
            digit = digit + 1;
        }
        return series;
    }
    series
}
