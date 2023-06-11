pub fn series(digits: &str, len: usize) -> Vec<String> {
    //unimplemented!("What are the series of length {len} in string {digits:?}")
    
    let digitsAsCharacters: Vec<char> = digits.chars().collect();
    let mut series: Vec<String> = Vec::new();
    let mut digit: usize = 0;
    
    while digit < digitsAsCharacters.len()
    {
        let mut digitsSet: usize = 0;
        let mut digitsSetElement: String = String::new();
        while digitsSet < len
        {
            digitsSetElement = digitsSetElement + &digitsAsCharacters[digit].to_string();
            digit = digit + 1;
            digitsSet = digitsSet + 1;
        }
        series.push(digitsSetElement);
    }
    series
}
