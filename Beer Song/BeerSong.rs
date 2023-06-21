pub fn verse(n: u32) -> String
{
    //unimplemented!("emit verse {n}")
    /*
    {n bottles, 1 bottle, No more bottles} of beer on the wall, {n bottles, 1 bottle, No more bottles} of beer.
    {Take {one, it} down and pass it around, Go to the store and buy some more}, {n-1 bottles, 1 bottle, no more bottles} of beer on the wall.
    */
    let mut verse: String = String::new();
    let mut firstInsert: String = String::new();
    let mut secondInsert: String = String::new();
    let mut thirdInsert: String = String::new();
    let mut fourthInsert: String = String::new();
    match n
    {
        0 =>
            {
                firstInsert = "No more bottles ".to_string();
                secondInsert = "no more bottles ".to_string();
                thirdInsert = "Go to the store and buy some more, ".to_string();
                fourthInsert = "99 bottles ".to_string();
            },
        1 =>
            {
                firstInsert = "1 bottle ".to_string();
                secondInsert = "1 bottle ".to_string();
                thirdInsert = "Take it down and pass it around, ".to_string();
                fourthInsert = "no more bottles ".to_string();
            },
        x =>
            {
                firstInsert = x.to_string() + " bottles ";
                secondInsert = x.to_string() + " bottles ";
                thirdInsert = "Take one down and pass it around, ".to_string();
                if n == 2
                {
                    fourthInsert = "1 bottle ".to_string();
                }
                else
                {
                    fourthInsert = (x - 1).to_string() + " bottles ";
                }
            },
    }
    verse = firstInsert + "of beer on the wall, ";
    verse = verse + &secondInsert;
    verse = verse + "of beer.\n";
    verse = verse + &thirdInsert;
    verse = verse + &fourthInsert;
    verse = verse + "of beer on the wall.\n";
    verse
}

pub fn sing(start: u32, end: u32) -> String
{
    //unimplemented!("sing verses {start} to {end}, inclusive")
    let mut song: String = String::new();
    let mut n: u32 = start;
    while n > end
    {
        song = song + &verse(n);
        song = song + "\n";
        n = n - 1;
    }
    song = song + &verse(end);
    println!("start {} end {}", start, end);
    println!("{}", song);
    song
}
