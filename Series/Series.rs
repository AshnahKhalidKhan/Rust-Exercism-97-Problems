pub fn series(digits: &str, len: usize) -> Vec<String> {
    //unimplemented!("What are the series of length {len} in string {digits:?}")
    
    let digitsAsCharacters: Vec<char> = digits.chars().collect();
    let mut series: Vec<String> = Vec::new();
    let mut digit: usize = 0;
    let mut digitsSet: usize = 0;

    println!("digits: {}", digits);
    println!("digitsAsCharacters: {:?}", digitsAsCharacters);
    println!("digitsAsCharacters.len(): {:?}", digitsAsCharacters.len());

    if digitsAsCharacters.len() < len
    {
        return series;
    }
    else if digitsAsCharacters.len() == len
    {
        series.push(digits.to_string());
        return series;
    }
    
    while digit < digitsAsCharacters.len()
    {
        println!("digit < digitsAsCharacters.len(): {} < {}", digit, digitsAsCharacters.len());
        let mut seriesElement: String = String::new();
        while digitsSet < len && (digit + digitsSet) < digitsAsCharacters.len()
        {
            println!("digit: {}, digitsSet: {}, digit+digitsSet: {}, ", digit, digitsSet, digit+digitsSet);
            seriesElement = seriesElement + &digitsAsCharacters[digit + digitsSet].to_string();
            digitsSet = digitsSet + 1;
        }
        println!("seriesElement: {}", seriesElement);
        if seriesElement.len() == len
        {
            series.push(seriesElement);
        }
        digit = digit + 1;
        digitsSet = 0;
    }
    series
}
