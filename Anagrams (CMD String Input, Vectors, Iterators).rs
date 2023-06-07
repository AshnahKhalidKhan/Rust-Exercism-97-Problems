/******************************************************************************
Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
use std::env;

fn main()
{
    let vectorWithCMDInputAndThatExtraString: Vec<String> = env::args().collect();
    println!("Raw CMD input: {:?}", vectorWithCMDInputAndThatExtraString);
    if vectorWithCMDInputAndThatExtraString.len() > 1
    {
        let wordToCheck = vectorWithCMDInputAndThatExtraString[1].to_string().to_lowercase().clone();
        println!("Word to check anagrams for: {}", wordToCheck);
        let mut vectorWithOnlyCMDInput: Vec<String> = Vec::new();
        
        let mut index: u32 = 0;
        for element in vectorWithCMDInputAndThatExtraString.iter()
        {
            if index > 1 && element.to_string().to_lowercase() != wordToCheck
            {
                vectorWithOnlyCMDInput.push(element.to_string().to_lowercase());
            }
            index = index + 1;
        }
        println!("Vector with only useful CMD input & no same words as word to check: {:?}", vectorWithOnlyCMDInput);
        
        let mut vectorWithSortedCharacterStrings: Vec<String> = Vec::new();
        for element in vectorWithOnlyCMDInput.iter()
        {
            println!("\nString element: {}", element);
            let mut x: Vec<char> = element.chars().collect();
            println!("String element's characters: {:?}", x);
            x.sort_unstable();
            let sortedCharacterString: String = x.into_iter().collect();
            println!("String element with sorted characters: {}", sortedCharacterString);
            vectorWithSortedCharacterStrings.push(sortedCharacterString);
        }
        
        let mut wordToChecksCharacters: Vec<char> = wordToCheck.chars().collect();
        println!("\nWord to check's characters: {:?}", wordToChecksCharacters);
        wordToChecksCharacters.sort_unstable();
        let wordToCheckWithSortedCharacters: String = wordToChecksCharacters.into_iter().collect();
        println!("Word to check with sorted characters niw: {}", wordToCheckWithSortedCharacters);
    
        let mut AnagramsOfWord: Vec<String> = Vec::new();
        
        for index in 0..vectorWithSortedCharacterStrings.len()
        {
            if vectorWithSortedCharacterStrings[index] == wordToCheckWithSortedCharacters
            {
                AnagramsOfWord.push(vectorWithOnlyCMDInput[index].clone());
            }
        }
        
        println!("Anagrams of given word '{}': {:?}", wordToCheck, AnagramsOfWord);
    }
}