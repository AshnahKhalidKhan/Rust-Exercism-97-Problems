pub fn annotate(minefield: &[&str]) -> Vec<String>
{
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    
    let rows: isize = minefield.len() as isize;
    let columns: isize = minefield[0].len() as isize;

    println!("Minefield: {:?}", minefield);
    println!("Rows: {}", rows);
    println!("Columns: {}", columns);
    
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
            //println!("{}", ((i % columns) as isize - 1));
            if (i % columns) as isize - 1 >= 0 && allInOneRow[i - 1] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check right row
            if (i % columns) + 1 < columns && allInOneRow[i + 1] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check up row
            if (i - rows) as isize >= 0 && allInOneRow[i - rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check down row
            if (i + rows) < allInOneRow.len() && allInOneRow[i + rows] == &(42 as u8)
            {
                count = count + 1;
            }

            //Check left up row (diagonal)
            if (i % columns) as isize - 1 >= 0 && i as isize - rows >= 0 && allInOneRow[i - 1 - rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check left down row (diagonal)
            if (i % columns) as isize - 1 >= 0 && i + rows < allInOneRow.len() && allInOneRow[i - 1 + rows] == &(42 as u8)
            {
                count = count + 1;
            }
            //Check right up row (diagonal)
            if (i % columns) + 1 < columns && i as isize - rows >= 0 && allInOneRow[i + 1 - rows] == &(42 as u8)
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
    let mut finalnumber: Vec<String> = Vec::new();
    let mut line: String = String::new();
    for i in 0..numbers.len()
    {
        if i != 0 && i % columns == 0
        {
            finalnumber.push(line);
            line = String::new();
            
        }
        line = line + &numbers[i];
    }
    finalnumber
    //numbers
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
