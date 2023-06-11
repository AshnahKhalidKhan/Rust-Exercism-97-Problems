pub fn brackets_are_balanced(string: &str) -> bool {
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");
    
    let mut separateCharacters: Vec<char> = string.chars().collect();
    let mut bracketsMatch: bool = true;

    
    println!("{:?}", separateCharacters);
    while separateCharacters.len() != 0 || bracketsMatch != false
    {
        for index in 0..separateCharacters.len()
        {
            match (separateCharacters[0], separateCharacters[index])
            {
                ('}', _) =>
                        {
                            bracketsMatch = false;
                            break;
                        },
                (']', _) =>
                        {
                            bracketsMatch = false;
                            break;
                        },
                (')', _) =>
                        {
                            bracketsMatch = false;
                            break;
                        },
                ('{', '}') =>
                        {
                            separateCharacters.remove(index);
                            separateCharacters.remove(0);
                            break;
                        },
                ('[', ']') =>
                        {
                            separateCharacters.remove(index);
                            separateCharacters.remove(0);
                            break;
                        },
                ('(', ')') =>
                        {
                            separateCharacters.remove(index);
                            separateCharacters.remove(0);
                            break;
                        },   
                (_,_) =>
                        {
                            println!("I don't know bro");
                        },
            }
            println!("{}", index);
        }
    }
    bracketsMatch
}
