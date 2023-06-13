pub fn reply(message: &str) -> &str
{
    //unimplemented!("have Bob reply to the incoming message: {message}")

    let mut Silence: bool = false;
    let mut QuestionMarkAtEnd: bool = false;
    let mut ALLCAPS: bool = true;

    let mut messageBreakdown: Vec<char> = message.chars().collect();
    let mut messageWithoutWhitespaces: Vec<char> = Vec::new();
    let mut messageWithOnlyAlphabets: Vec<char> = Vec::new();

    for index in 0..messageBreakdown.len()
    {
        //If it's not a whitespace, add it to messageWithoutWhitespaces vector
        if messageBreakdown[index] != '\0' && messageBreakdown[index].is_whitespace() == false
        {
            messageWithoutWhitespaces.push(messageBreakdown[index]);
        }
        //If it is an alphabet, add it to messageWithOnlyAlphabets vector
        if messageBreakdown[index].is_ascii_alphabetic() == true
        {
            messageWithOnlyAlphabets.push(messageBreakdown[index]);
        }
    }

    //If the entire message was composed of whitespaces, messageWithoutWhitespaces will be empty
    if messageWithoutWhitespaces.len() == 0
    {
        Silence = true;
    }
    //If the message had some characters, did it end with a question mark?
    else if messageWithoutWhitespaces[messageWithoutWhitespaces.len() - 1] == '?'
    {
        QuestionMarkAtEnd = true;
    }

    //If the message had some alphabets, check if they were all capital letters
    if messageWithOnlyAlphabets.len() != 0
    {
        for index in 0..messageWithOnlyAlphabets.len()
        {
            //If even one of the alphabets in the message is not capitalized, ALLCAPS is false
            if messageWithOnlyAlphabets[index] != messageWithOnlyAlphabets[index].to_ascii_uppercase()
            {
                ALLCAPS = false;
                break;
            }
        }
    }
    //If there were no alphabets in the message, ALLCAPS can't be true
    else if messageWithOnlyAlphabets.len() == 0
    {
        ALLCAPS = false;
    }

    // println!("messageBreakdown: {:?}", messageBreakdown);
    // println!("messageWithoutWhitespaces: {:?}", messageWithoutWhitespaces);
    // println!("Silence: {}", Silence);
    // println!("QuestionMarkAtEnd: {}", QuestionMarkAtEnd);
    // println!("messageWithOnlyAlphabets: {:?}", messageWithOnlyAlphabets);
    // println!("ALLCAPS: {}", ALLCAPS);

    match (Silence, QuestionMarkAtEnd, ALLCAPS)
    {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
