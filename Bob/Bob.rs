pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {message}")
    let mut messageBreakdown: Vec<char> = message.chars().collect();
    println!("{:?}", messageBreakdown);
    let mut QuestionMarkAtEnd: bool = match messageBreakdown[messageBreakdown.len() - 1]
                                    {
                                        '?' => true,
                                        _ => false,
                                    };
    println!("QuestionMarkAtEnd: {}", QuestionMarkAtEnd);

    let mut ALLCAPS: bool = true;
    for character in message.chars()
    {
        if character.is_ascii_alphabetic() == true && character != character.to_ascii_uppercase()
        {
            ALLCAPS = false;
            break;
        }
    }
    println!("AllCaps: {}", ALLCAPS);

    let mut Silence: bool = false;
    //Sophisticated answer would be to use this and just be done with it: message.chars().filter(|c| !c.is_whitespace()).collect()
    let mut NotWhitespace: Vec<char> = Vec::new();
    for index in 0..messageBreakdown.len()
    {
        if messageBreakdown[messageBreakdown.len() - 1 - index] != '\0' && messageBreakdown[messageBreakdown.len() - 1 - index].is_whitespace() == false
        {
            NotWhitespace.push(messageBreakdown[messageBreakdown.len() - 1 - index]);
        }
    }
    if NotWhitespace.len() == 0
    {
        Silence = true;
    }
    else
    {
        Silence = false;
    }
    println!("Silence: {}", Silence);
    match (Silence, QuestionMarkAtEnd, ALLCAPS)
    {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
