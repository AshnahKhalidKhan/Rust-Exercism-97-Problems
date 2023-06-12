pub fn brackets_are_balanced(string: &str) -> bool {
    //unimplemented!("Check if the string \"{string}\" contains balanced brackets");
    
    let SeparateCharacters: Vec<char> = string.chars().collect();
    let mut BracketsOnly: Vec<char> = Vec::new();
    for c in 0..SeparateCharacters.len()
    {
        if SeparateCharacters[c] == '{' || SeparateCharacters[c] == '}' || SeparateCharacters[c] == '[' || SeparateCharacters[c] == ']' || SeparateCharacters[c] == '(' || SeparateCharacters[c] == ')'
        {
            BracketsOnly.push(SeparateCharacters[c]);
        }
    }
    println!("{:?}", SeparateCharacters);
    println!("{:?}", BracketsOnly);
    
    let mut bracketsMatch: bool = true;
    let mut currentbracket: usize = 0;
    //let mut backwardbracket: usize = BracketsOnly.len() - 1;

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
        bracketsMatch = false;
    }
    else
    {
        bracketsMatch = true;
    }
    bracketsMatch

    /*
    while currentbracket <= backwardbracket
    {
        println!("{} {}", BracketsOnly[currentbracket], BracketsOnly[backwardbracket]);
        if BracketsOnly[currentbracket] == '}' || BracketsOnly[currentbracket] == ']' || BracketsOnly[currentbracket] == ')'
        {
            bracketsMatch = false;
            break;
        }
        else if (BracketsOnly[currentbracket] == '{' && BracketsOnly[backwardbracket] == '}') || (BracketsOnly[currentbracket] == '[' && BracketsOnly[backwardbracket] == ']') || (BracketsOnly[currentbracket] == '(' && BracketsOnly[backwardbracket] == ')')
        {
            BracketsOnly.remove(backwardbracket);
            BracketsOnly.remove(currentbracket);
            currentbracket = 0;
            backwardbracket = BracketsOnly.len() - 1;
        }
        else if currentbracket == backwardbracket
        {
            bracketsMatch = false;
            break;
        }
        else
        {
            backwardbracket = backwardbracket - 1;
        }
    }*/
    
    /*while BracketsOnly.len() != 0 || bracketsMatch != false
    {
        for index in 0..BracketsOnly.len()
        {
            if index == 0
            {
                bracketsMatch = false;
                break;
            }
            match (BracketsOnly[0], BracketsOnly[BracketsOnly.len() - 1 - index])
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
                            BracketsOnly.remove(BracketsOnly.len() - 1 - index);
                            BracketsOnly.remove(0);
                            break;
                        },
                ('[', ']') =>
                        {
                            BracketsOnly.remove(BracketsOnly.len() - 1 - index);
                            BracketsOnly.remove(0);
                            break;
                        },
                ('(', ')') =>
                        {
                            BracketsOnly.remove(BracketsOnly.len() - 1 - index);
                            BracketsOnly.remove(0);
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
    bracketsMatch*/
}
