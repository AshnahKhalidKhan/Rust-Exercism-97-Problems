pub fn annotate(minefield: &[&str]) -> Vec<String>
{
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    println!("{:?}", minefield);
    let rows = minefield.len();
    println!("{}", rows);
    let columns = minefield[0].len();
    println!("{}", columns);
    let mut allInOneRow: Vec<&u8> = Vec::new();
    for line in minefield.iter()
    {
        println!("{}", line);
        for i in line.as_bytes()
        {
            allInOneRow.push(i);
        }
    }
    let mut numbers: Vec<String> = Vec::new();
    for i in 0..allInOneRow.len()
    {
        if allInOneRow[i] == &(42 as u8)
        {
            numbers.push("*".to_string());
        }
        else
        {
            let mut count: u64 = 0;
            
            //Check left up row (diagonal)
            //Check right up row (diagonal)
            //Check left down row (diagonal)
            //Check right down row (diagonal)
            
            //Check left row
            if (i % columns) - 1 >= 0 && allInOneRow[i - 1] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check right row
            if (i % columns) + 1 < columns && allInOneRow[i + 1] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check up row
            if (i - rows) >= 0 && allInOneRow[i - rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check down row
            if (i + rows) < allInOneRow.len() && allInOneRow[i + rows] == &(42 as u8)
            {
                count = count + 1;
            }

            //Check left up row (diagonal)
            if (i % columns) - 1 >= 0 && i - rows >= 0 && allInOneRow[i - 1 - rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check left down row (diagonal)
            if (i % columns) - 1 >= 0 && i + rows < allInOneRow.len() && allInOneRow[i - 1 + rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check right up row (diagonal)
            if (i % columns) + 1 < columns && i - rows >= 0 && allInOneRow[i + 1 - rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check right down row (diagonal)
            if (i % columns) + 1 < columns && i + rows < allInOneRow.len() && allInOneRow[i + 1 + rows] == &(42 as u8)
            {
                count = count + 1;
            }
            if count > 0
            {
                numbers.push(count.to_string());
            }
            else
            {
                numbers.push('\u{a0}'.to_string());
            }
        }
    }
    println!("{:?}", allInOneRow);
    numbers
    /*
         0  1  2  3  4
         5  6  7  8  9
        10 11 12 13 14
        15 16 17 18 19
        20 21 22 23 24

        0  1  2  3  4
        0  1  2  3  4
        0  1  2  3  4
        0  1  2  3  4
        0  1  2  3  4
    */
}
