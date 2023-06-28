pub fn annotate(minefield: &[&str]) -> Vec<String>
{
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    println!("Minefield: {:?}", minefield);
    
    let mut markedMine: Vec<String> = Vec::new();
    if minefield.len() == 0
    {
        return markedMine;
    }
    
    let rows = minefield.len();
    let columns = minefield[0].len();

    if columns == 0
    {
        markedMine.push(minefield[0].to_string());
        return markedMine;
    }

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
    println!("allInOneRow: {:?}", allInOneRow);
    
    let mut numbers: Vec<String> = Vec::new();
    for i in 0..allInOneRow.len()
    {
        if allInOneRow[i] == &(42 as u8)
        {
            numbers.push("*".to_string());
        }
        else
        {
            println!("i={}", i);
            let mut count: u64 = 0;
            let left: isize = ((i % columns) as isize) - 1;
            if left >= 0 && allInOneRow[(i as isize - 1) as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let right: usize = (i % columns) + 1;
            if right < columns && allInOneRow[i + 1] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let up: isize = (i as isize) - (columns as isize);
            if up >= 0 && allInOneRow[up as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let down: usize = i + columns;
            if down < allInOneRow.len() && allInOneRow[down] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let leftUp: isize = (i as isize) - 1 - (columns as isize);
            if left >= 0 && up >= 0 && allInOneRow[leftUp as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let rightUp: isize = ((i + 1) as isize) - (columns as isize);
            if right < columns && up >= 0 && allInOneRow[rightUp as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let leftDown: isize = (i as isize) - 1 + (columns as isize);
            if leftDown >= 0 && (leftDown as usize) < allInOneRow.len() && leftDown % (columns as isize) < (i % columns) as isize && allInOneRow[leftDown as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            
            let rightDown: usize = i + 1 + columns;
            if rightDown >= 0 && rightDown < allInOneRow.len() && rightDown % columns > i % columns && allInOneRow[rightDown] == &(42 as u8)
            {
                count = count + 1;
            }

            match count
            {
                0 => numbers.push(" ".to_string()),
                _ => numbers.push(count.to_string())
            };
        }
    }
    println!("numbers: {:?}", numbers);
    
    let mut line: String = String::new();
    let mut count: usize = 0;
    for i in 0..numbers.len()
    {
        line = line + &numbers[i];
        if count == columns - 1
        {
            markedMine.push(line);
            count = 0;
            line = String::new();
        }
        else
        {
            count = count + 1;
        }
    }
    markedMine
    
    //numbers
    /*
        let left: isize = ((i % columns) as isize) - 1;
        let right: usize = (i % columns) + 1;
         0  1  2  3  4
         5  6  7  8  9
        10 11 12 13 14
        15 16 17 18 19
        20 21 22 23 24

        0  1  2  3  4                0  1  2       0  1  2   
        0  1  2  3  4                0  *  2       0  1  2    
        0  1  2  3  4                0  1  2       0  1  2    
        0  1  2  3  4
        0  1  2  3  4

        ((i + columns) + 1) % columns < columns
        0  1  *  3  4
        0  1  *  3  4
        *  *  *  *  *
        0  1  *  3  4
        0  1  *  3  4
    */
}
