pub fn series(digits: &str, len: usize) -> Vec<String> {
    //unimplemented!("What are the series of length {len} in string {digits:?}")
    
    let digitsAsCharacters: Vec<char> = digits.chars().collect();
    let mut series: Vec<String> = Vec::new();
    let mut digit: usize = 0;

    println!("{}", digits);
    println!("{:?}", digitsAsCharacters);

    while digit < digitsAsCharacters.len()
    {
        let mut digitsSet: usize = 0;
        let mut digitsSetElement: String = String::new();
        while digitsSet < len && (digit + digitsSet) < digitsAsCharacters.len()
        {
            println!("digit: {}, digitsSet: {}, digit+digitsSet: {}, ", digit, digitsSet, digit+digitsSet);
            digitsSetElement = digitsSetElement + &digitsAsCharacters[digit + digitsSet].to_string();
            digitsSet = digitsSet + 1;
        }
        println!("digitsetElement: {}", digitsSetElement);
        if digitsSetElement.len() == len
        {
            series.push(digitsSetElement);
        }
        digit = digit + 1;
    }
    series
}
