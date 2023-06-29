pub fn abbreviate(phrase: &str) -> String
{
    //unimplemented!("Given the phrase '{phrase}', return its acronym");

    //Split characters by whitespace and punctuation marks except apostrophes
    let mut words: Vec<&str> = phrase.split(|c: char| c.is_alphabetic() == false && c != '\'' || c.is_whitespace()).collect();
    
    let mut firstLettersCapitalized: String = String::new();

    for word in words
    {
        //If the word is not an empty string
        if word.len() != 0
        {
            let mut characters: Vec<char> = word.chars().collect();
            let mut upperCasedCharacters: Vec<char> = word.chars().map(|c| c.to_ascii_uppercase()).collect();

            //Check if the entire string is in uppercase
            if characters == upperCasedCharacters
            {
                //Keep the first capital letter only
                firstLettersCapitalized = firstLettersCapitalized + &upperCasedCharacters[0].to_string();
            }
            else
            {
                //Capitalize the first letter, and keep the original form of the rest of the letters
                characters[0] = characters[0].to_ascii_uppercase();
                firstLettersCapitalized = firstLettersCapitalized + &characters.iter().collect::<String>();
            }
        }
    }

    let mut acronym: String = String::new();
    for c in firstLettersCapitalized.chars()
    {
        //If a letter is capital, it is part of the acronym
        if c.is_uppercase() == true
        {
            acronym = acronym + &c.to_string();
        }
    }
    acronym
}
