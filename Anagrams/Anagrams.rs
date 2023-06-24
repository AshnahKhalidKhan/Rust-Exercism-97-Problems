use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>
{
    //unimplemented!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut Anagrams: HashSet<&str> = HashSet::new();
    let mut wordLowerCase: Vec<char> = word.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut wordRearranged: Vec<char> = wordLowerCase.clone();
    wordRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap());
    println!("wordRearranged: {:?}", wordRearranged);
    for i in possible_anagrams.iter()
    {
        let mut allLowerCase: Vec<char> = i.chars().map(|c| c.to_ascii_lowercase()).collect();
        let mut lowerCaseRearranged: Vec<char> = allLowerCase.clone();
        lowerCaseRearranged.sort_unstable_by(|firstLetter, nextLetter| firstLetter.partial_cmp(nextLetter).unwrap());
        println!("lowerCaseRearranged: {:?}", lowerCaseRearranged);
        if wordLowerCase != allLowerCase && lowerCaseRearranged == wordRearranged
        {
            Anagrams.insert(i);
        }
    }
    Anagrams
}
