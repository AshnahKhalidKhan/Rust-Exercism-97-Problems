pub fn abbreviate(phrase: &str) -> String
{
    //unimplemented!("Given the phrase '{phrase}', return its acronym");
    let mut acronym: String = String::new();
    for word in phrase.split_whitespace()
    {
        acronym = acronym + word.chars().next();
    }
    acronym
}
