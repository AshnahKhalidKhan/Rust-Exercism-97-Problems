pub fn abbreviate(phrase: &str) -> String
{
    //unimplemented!("Given the phrase '{phrase}', return its acronym");
    println!("phrase: {}", phrase);
    let mut acronym: String = String::new();
    println!("phrase.split_whitespace(): {:?}", phrase.split_whitespace());
    for word in phrase.split_whitespace()
    {
        acronym = acronym + &word.chars().next().unwrap().to_string();
    }
    println!("acronym: {}", acronym);
    acronym
}
