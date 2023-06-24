use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>
{
    //unimplemented!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut Anagrams: HashSet<&str> = HashSet::new();
    let mut wordRearranged: Vec<char> = word.chars().map(|c| c.to_ascii_lowercase()).collect();
    wordRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap()).collect();
    for i in possible_anagrams.iter()
    {
        let mut allLowerCase: Vec<char> = i.chars().map(|c| c.to_ascii_lowercase()).collect();
        if word.chars().map(|c| c.to_ascii_lowercase()).collect() != allLowerCase
        {
            allLowerCase.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap()).collect();
            if allLowerCase == wordRearranged
            {
                Anagrams.insert(i);
            }
        }
    }
    Anagrams
}
