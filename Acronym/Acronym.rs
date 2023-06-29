pub fn abbreviate(phrase: &str) -> String
{
    //unimplemented!("Given the phrase '{phrase}', return its acronym");
    println!("phrase: {}", phrase);
    
    let mut acronym: String = String::new();

    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    // let mut words: Vec<&str> = phrase.split(|c: char| c.is_alphabetic() == false || c.is_whitespace()).collect();
    println!("words: {:?}", words);

    for i in words
    {
        // let characters: Vec<char> = i.chars().map(|c: char| if c.is_uppercase() == true
        // {c}
        //                                          else
        // {continue}).collect();
        // println!("characters: {:?}", characters);
        // let mut nextCharacterAdd: bool = true;
        // for c in 0..characters.len()
        // {
        //     if c == 0 || c.is_alphabetic() == false || 
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
    
    println!("acronym: {}", acronym);
    
    acronym
}
