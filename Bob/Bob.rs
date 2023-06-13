pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {message}")
    // let mut Silence: bool = false;
    //     let mut messageBreakdown: Vec<char> = message.chars().collect();
    // println!("{:?}", messageBreakdown);
    // //Sophisticated answer would be to use this and just be done with it: message.chars().filter(|c| !c.is_whitespace()).collect()
    // let mut NotWhitespace: Vec<char> = Vec::new();
    // for index in 0..messageBreakdown.len()
    // {
    //     if (messageBreakdown[index] != '\0' && messageBreakdown[index].is_whitespace() == false && messageBreakdown[index].is_ascii_alphabetic() == true) || messageBreakdown[index] == '?'
    //     {
    //         NotWhitespace.push(messageBreakdown[index]);
    //     }
    // }

    // println!("NotWhiteSpace: {:?}", NotWhitespace);
    
    // if NotWhitespace.len() == 0
    // {
    //     Silence = true;
    // }
    // else
    // {
    //     Silence = false;
    // }
    // println!("Silence: {}", Silence);

    // let mut QuestionMarkAtEnd: bool = match NotWhitespace[NotWhitespace.len() - 1]
    //                                 {
    //                                     '?' => true,
    //                                     _ => false,
    //                                 };
    // println!("QuestionMarkAtEnd: {}", QuestionMarkAtEnd);

    // let mut ALLCAPS: bool = true;
    // for character in message.chars()
    // {
    //     if character.is_ascii_alphabetic() == true && character != character.to_ascii_uppercase()
    //     {
    //         ALLCAPS = false;
    //         break;
    //     }
    // }
    // println!("AllCaps: {}", ALLCAPS);
    
    // match (Silence, QuestionMarkAtEnd, ALLCAPS)
    // {
    //     (true, _, _) => "Fine. Be that way!",
    //     (_, true, true) => "Calm down, I know what I'm doing!",
    //     (_, true, _) => "Sure.",
    //     (_, _, true) => "Whoa, chill out!",
    //     _ => "Whatever.",
    // }

    let mut Silence: bool = false;
    let mut QuestionMarkAtEnd: bool = false;
    let mut ALLCAPS: bool = true;

    let mut messageBreakdown: Vec<char> = message.chars().collect();
    let mut messageBreakdownWithoutWhitespaces: Vec<char> = Vec::new();
    let mut messageBreakdownWithOnlyAlphabets: Vec<char> = Vec::new();

    for index in 0..messageBreakdown.len()
    {
        if messageBreakdown[index] != '\0' && messageBreakdown[index].is_whitespace() == false
        {
            messageBreakdownWithoutWhitespaces.push(messageBreakdown[index]);
        }
    }
    if messageBreakdownWithoutWhitespaces.len() == 0
    {
        Silence = true;
    }
    else if messageBreakdownWithoutWhitespaces[messageBreakdownWithoutWhitespaces.len() - 1] == '?'
    {
        QuestionMarkAtEnd = true;
    }

    for index in 0..messageBreakdown.len()
    {
        if messageBreakdown[index].is_ascii_alphabetic() == true
        {
            messageBreakdownWithOnlyAlphabets.push(messageBreakdown[index]);
        }
    }
    
    if messageBreakdownWithOnlyAlphabets.len() != 0
    {
        for index in 0..messageBreakdownWithOnlyAlphabets.len()
        {
            if messageBreakdownWithOnlyAlphabets[index] != messageBreakdownWithOnlyAlphabets[index].to_ascii_uppercase()
            {
                ALLCAPS = false;
                break;
            }
        }
    }
    else if messageBreakdownWithOnlyAlphabets.len() == 0
    {
        ALLCAPS = false;
    }

    println!("messageBreakdown: {:?}", messageBreakdown);
    println!("messageBreakdownWithoutWhitespaces: {:?}", messageBreakdownWithoutWhitespaces);
    println!("Silence: {}", Silence);
    println!("QuestionMarkAtEnd: {}", QuestionMarkAtEnd);
    println!("messageBreakdownWithOnlyAlphabets: {:?}", messageBreakdownWithOnlyAlphabets);
    println!("ALLCAPS: {}", ALLCAPS);

    match (Silence, QuestionMarkAtEnd, ALLCAPS)
    {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
