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
            if i.len() == word.len()
            {
                let mut allLowerCase: Vec<ToLowercase> = word.chars().map(|c| c.to_lowercase()).collect();
            
            let mut sameWord: bool = true;
            for n in 0..wordLowerCase.len()
            {
                if wordLowerCase[n] != allLowerCase[n]
                {
                    sameWord = false;
                    break;
                }
            }
            if sameWord == false
            {
                allLowerCase.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(*nextLetter).unwrap());
                let mut sameArrangement: bool = true;
                for n in 0..wordRearranged.len()
                {
                    if wordRearranged[n] != allLowerCase[n]
                    {
                        sameArrangement = false;
                        break;
                    }
                }
                if sameArrangement == true
                {
                    Anagrams.insert(i);
                }
            }
        }
    }
    Anagrams
}
