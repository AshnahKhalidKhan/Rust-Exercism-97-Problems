pub fn abbreviate(phrase: &str) -> String
{
    //unimplemented!("Given the phrase '{phrase}', return its acronym");
    println!("phrase: {}", phrase);
    
    let mut acronym: String = String::new();

    // let mut words: Vec<&str> = phrase.split_whitespace().collect();
    let mut words: Vec<&str> = phrase.split(|c: char| c.is_alphabetic() == false && c != '\'' || c.is_whitespace()).collect();
    println!("words: {:?}", words);

    let mut newPhrase: String = String::new();

    for i in words
    {
        
        let mut characters: Vec<char> = i.chars().collect();
        if characters.len() > 0
        {
            characters[0] = characters[0].to_ascii_uppercase();
        }
        let mut allUpperCase: bool = true;
        for x in characters.iter()
        {
            if x.is_alphabetic() == true && x.is_uppercase() == false
            {
                allUpperCase = false;
                break;
            }
        }

        if allUpperCase == true && characters.len() > 0
        {
            characters = i.chars().map(|c| c.to_ascii_lowercase()).collect();
            characters[0] = characters[0].to_ascii_uppercase();
        }
        
        let newString: String = characters.iter().collect::<String>();
        newPhrase = newPhrase + &newString;
        println!("characters: {:?}", characters);
        println!("newString: {}", newString);
        println!("newPhrase: {:?}", newPhrase);
        
        
        // println!("characters: {:?}", characters);
        
        // let mut nextCharacterAdd: bool = true;
        // for c in 0..characters.len()
        // {
        //     match (c, character[c].is_alphabetic(), nextCharacterAdd, character[c].is_uppercase(), c+1 < characters.len())
        //     {
        //         //first character, a character following a non alphabet, or an uppercase character
        //         (0, _, _, _, _) || (_, _, true, _, _) =>
        //         {
        //             acronym = acronym + &(characters[c].to_uppercase()).to_string();
        //             nextCharacterAdd = false;
        //         },
        //         (_, false, _, _
        //         (_, _, _) => println!("ua")
        //     };
        // }
                
        // if i.chars().map(|c| if c.is_alphabetic() == true {c.to_ascii_uppercase()}).collect() == i.chars().map(|c| if c.is_alphabetic() == true {c}).collect()
        // {
        //     let firstCharacter: char = i.chars().next().unwrap().to_ascii_uppercase();
        //     acronym = acronym + &firstCharacter.to_string();
        // }
    }
    
    //println!("phrase.split_whitespace(): {:#?}", phrase.split_whitespace());
    
    //phrase.split_whitespace();
    // for word in phrase.split_whitespace()
    // {
    //     println!("{}", word);
    //     let firstCharacter: char = word.chars().next().unwrap().to_ascii_uppercase();
    //     if firstCharacter.is_alphabetic()
    //     {
    //         acronym = acronym + &firstCharacter.to_string();
    //     }
    // }

    for c in newPhrase.chars()
    {
        if c.is_uppercase() == true
        {
            acronym = acronym + &c.to_string();
        }
    }
    
    println!("acronym: {}", acronym);
    
    acronym
}
