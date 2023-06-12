pub fn brackets_are_balanced(string: &str) -> bool
{
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");
    
    let SeparateCharacters: Vec<char> = string.chars().collect();
    let BracketsPool: Vec<char> = vec!['{', '}', '[', ']', '(', ')'];
    let mut BracketsOnly: Vec<char> = Vec::new();
    for c in 0..SeparateCharacters.len()
    {
        //if SeparateCharacters[c] == '{' || SeparateCharacters[c] == '}' || SeparateCharacters[c] == '[' || SeparateCharacters[c] == ']' || SeparateCharacters[c] == '(' || SeparateCharacters[c] == ')'
        if BracketsPool.contains(&SeparateCharacters[c])
        {
            BracketsOnly.push(SeparateCharacters[c]);
        }
    }
    
    let mut currentbracket: usize = 0;
    while (currentbracket + 1) < BracketsOnly.len()
    {
        if (BracketsOnly[currentbracket] == '{' && BracketsOnly[currentbracket + 1] == '}') || (BracketsOnly[currentbracket] == '[' && BracketsOnly[currentbracket + 1] == ']') || (BracketsOnly[currentbracket] == '(' && BracketsOnly[currentbracket + 1] == ')')
        {
            BracketsOnly.remove(currentbracket + 1);
            BracketsOnly.remove(currentbracket);
            currentbracket = 0;
        }
        else
        {
            currentbracket = currentbracket + 1;
        }
    }
    
    if BracketsOnly.len() != 0
    {
        return false;
    }
    else
    {
        return true;
    }
}
