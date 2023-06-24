use std::collections::HashSet;
use std::char::ToLowercase;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>
{
    //unimplemented!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut Anagrams: HashSet<&str> = HashSet::new();
    let mut wordLowerCase: Vec<ToLowercase> = word.chars().map(|c| c.to_lowercase()).collect();
    let mut wordRearranged: Vec<ToLowercase> = wordLowerCase.clone();
    wordRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(*nextLetter).unwrap());
    println!("wordRearranged: {:?}", wordRearranged);
    for i in possible_anagrams.iter()
    {
            let mut allLowerCase: Vec<ToLowercase> = word.chars().map(|c| c.to_lowercase()).collect();
    let mut lowerCaseRearranged: Vec<ToLowercase> = wordLowerCase.clone();
    lowerCaseRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(*nextLetter).unwrap());
        println!("lowerCaseRearranged: {:?}", lowerCaseRearranged);
        if wordLowerCase.iter() != allLowerCase.iter() && lowerCaseRearranged.iter() == wordRearranged.iter()
        {
            Anagrams.insert(i);
        }
    }
    Anagrams
}
