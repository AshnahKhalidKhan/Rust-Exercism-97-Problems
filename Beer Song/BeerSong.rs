pub fn verse(n: u32) -> String {
    //unimplemented!("emit verse {n}")
    let mut verse: String = String::new();
    if n == 0
    {
        verse = verse + "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.";
    }
    else if n == 1
    {
        verse = verse + "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.";
    }
    else 
    {
        verse = verse + n + " bottles of beer on the wall, " + n + " bottles of beer.\nTake it down and pass it around, ";
        if n == 2
        {
            verse = verse + "1 bottle of beer on the wall."
        }
        else
        {
            verse = verse + (n - 1) + " bottles of beer on the wall."
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {start} to {end}, inclusive")
    let mut song: String = String::new();
    for n in start..=end
    {
        song = song + verse(n);
        song = song + "\n\n";
    }
    song
}
