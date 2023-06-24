use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str>
{
    //unimplemented!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut Anagrams: HashSet<&[&str]> = HashSet::new();
    for i in possible_anagrams.iter()
    {
        let mut allLowerCase: Vec<char> = i.chars().map(|c| c.to_ascii_lowercase()).collect();
        allLowerCase.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    }
    HashSet::new()
}
