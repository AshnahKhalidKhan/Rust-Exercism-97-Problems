pub fn annotate(minefield: &[&str]) -> Vec<String>
{
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    
    let rows = minefield.len();
    let columns = minefield[0].len();

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
            println!("i={}", i);
            let mut count: u64 = 0;
            let left: isize = ((i % columns) as isize) - 1;
            let right: usize = (i % columns) + 1;
            let up: isize = (i as isize) - (columns as isize);
            let down: usize = i + columns;
            let leftUp: isize = (i as isize) - 1 - (columns as isize);
            let rightUp: isize = ((i + 1) as isize) - (columns as isize);
            let leftDown: isize = (i as isize) - 1 + (columns as isize);
            let rightDown: usize = i + 1 + columns;
            if left >= 0 && allInOneRow[(i as isize - 1) as usize] == &(42 as u8)
            {
                println!("left par hai: {} count:{}", left, count);
                count = count + 1;
            }
            if right < columns && allInOneRow[i + 1] == &(42 as u8)
            {
                println!("right par hai: {} count:{}", right, count);
                count = count + 1;
            }
            if up >= 0 && allInOneRow[up as usize] == &(42 as u8)
            {
                println!("up par hai: {} count:{}", up, count);
                count = count + 1;
            }
            if down < allInOneRow.len() && allInOneRow[down] == &(42 as u8)
            {
                println!("down par hai: {} count:{}", down, count);
                count = count + 1;
            }
            // if left >= 0 && up >= 0 && allInOneRow[leftUp as usize] == &(42 as u8)
            // {
            //     println!("leftUp par hai: {} count:{}", leftUp, count);
            //     count = count + 1;
            // }
            // if right < columns && up >= 0 && allInOneRow[rightUp as usize] == &(42 as u8)
            // {
            //     println!("rightUp par hai: {} count:{}", rightUp, count);
            //     count = count + 1;
            // }
            // if left >= 0 && down < columns && allInOneRow[leftDown as usize] == &(42 as u8)
            // {
            //     println!("leftDown par hai: {} count:{}", leftDown, count);
            //     count = count + 1;
            // }
            // if right < columns && down < columns && allInOneRow[rightDown] == &(42 as u8)
            // {
            //     println!("rightDown par hai: {} count:{}", rightDown, count);
            //     count = count + 1;
            // }

            if left >= 0 && up >= 0 && allInOneRow[leftUp as usize] == &(42 as u8)
            {
                println!("leftUp par hai: {} count:{}", leftUp, count);
                count = count + 1;
            }
            if right < columns && up >= 0 && allInOneRow[rightUp as usize] == &(42 as u8)
            {
                println!("rightUp par hai: {} count:{}", rightUp, count);
                count = count + 1;
            }
            if leftDown >= 0 && (leftDown as usize) < allInOneRow.len() && leftDown % (columns as isize) < (i % columns) as isize && allInOneRow[leftDown as usize] == &(42 as u8)
            {
                println!("leftDown par hai: {} count:{}", leftDown, count);
                count = count + 1;
            }
            if rightDown >= 0 && rightDown < allInOneRow.len() && rightDown % columns > i % columns && allInOneRow[rightDown] == &(42 as u8)
            {
                println!("rightDown par hai: {} count:{}", rightDown, count);
                count = count + 1;
            }
            
            if count > 0
            {
                numbers.push(count.to_string());
            }
            else
            {
                numbers.push(" ".to_string());
            }
        }
    }
    println!("allInOneRow: {:?}", allInOneRow);
    println!("numbers: {:?}", numbers);
    
    let mut finalnumber: Vec<String> = Vec::new();
    let mut line: String = String::new();
    let mut count: usize = 0;
    for i in 0..numbers.len()
    {
        line = line + &numbers[i];
        if count == columns - 1
        {
            finalnumber.push(line);
            count = 0;
            line = String::new();
        }
        else
        {
            count = count + 1;
        }
    }
    finalnumber
    
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
