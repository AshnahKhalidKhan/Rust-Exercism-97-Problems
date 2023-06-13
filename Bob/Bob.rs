pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {message}")
    let mut messageBreakdown: Vec<char> = message.chars().collect();
    println!("{:?}", messageBreakdown);
    let mut QuestionMarkAtEnd: bool = match messageBreakdown[messageBreakdown.len() - 1]
                                    {
                                        '?' => true,
                                        _ => false,
                                    };

    let mut ALLCAPS: bool = true;
    for character in message.chars()
    {
        if character.is_ascii_alphabetic == true && character == character.to_ascii_uppercase()
        {
            ALLCAPS = false;
            break;
        }
    }

    let mut Silence: bool = true;
    //Sophisticated answer would be to use this and just be done with it: message.chars().filter(|c| !c.is_whitespace()).collect()
    for index in 0..messageBreakdown.len()
    {
        if messageBreakdown[messageBreakdown.len() - 1 - index] == '' || messageBreakdown[messageBreakdown.len() - 1 - index].is_whitespace() == true
        {
            messageBreakdown.remove(messageBreakdown.len() - 1 - index);
        }
    }
    if messageBreakdown.len() == 0
    {
        Silence = true;
    }
    else
    {
        Silence = false;
    }
    
    match (Silence, QuestionMarkAtEnd, ALLCAPS)
    {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
