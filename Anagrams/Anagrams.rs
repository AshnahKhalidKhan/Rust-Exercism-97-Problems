use std::collections::HashSet;
use std::char::ToLowercase;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>
{
    //unimplemented!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    
    let mut Anagrams: HashSet<&str> = HashSet::new();
    
    let mut wordLowerCase: Vec<String> = word.chars().map(|c| c.to_lowercase().to_string()).collect();
    
    let mut wordRearranged: Vec<String> = wordLowerCase.clone();
    wordRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap());
    
    for i in possible_anagrams.iter()
    {
        if i.len() == word.len()
        {
            let mut allLowerCase: Vec<String> = i.chars().map(|c| c.to_lowercase().to_string()).collect();
            if wordLowerCase != allLowerCase
            {
                allLowerCase.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap());
                if wordRearranged == allLowerCase
                {
                    Anagrams.insert(i);
                }
            }
        }
    }
    Anagrams
}
