pub fn brackets_are_balanced(string: &str) -> bool
{
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");

    //Remove any characters that are not brackets
    let SeparateCharacters: Vec<char> = string.chars().collect();
    let BracketsPool: Vec<char> = vec!['{', '}', '[', ']', '(', ')'];
    let mut BracketsOnly: Vec<char> = Vec::new();
    for c in 0..SeparateCharacters.len()
    {
        if BracketsPool.contains(&SeparateCharacters[c])
        {
            BracketsOnly.push(SeparateCharacters[c]);
        }
    }
    
    //Find valid bracket pairs and remove them
    //If you do not find a match for a bracket after traversing the whole array, or if you have removed all bracket pairs and no brackets are left to check, stop checking
    let mut currentbracket: usize = 0;
    while (currentbracket + 1) < BracketsOnly.len()
    {
        match (BracketsOnly[currentbracket], BracketsOnly[currentbracket + 1])
        {
            ('{', '}') | ('[', ']') | ('(', ')') =>
                {
                    BracketsOnly.remove(currentbracket + 1);
                    BracketsOnly.remove(currentbracket);
                    currentbracket = 0;
                },
            _ => currentbracket = currentbracket + 1
        };
    }

    //If there were only valid bracket pairs, then they were all removed and then length of the remaining brackets is zero, which means brackets are balanced so return true
    //If not, there are brackets left which did not have valid pairs and the brackets are unbalanced so return false
    if BracketsOnly.len() == 0
    {
        return true;
    }
    else
    {
        return false;
    }
}
