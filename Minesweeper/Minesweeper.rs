pub fn annotate(minefield: &[&str]) -> Vec<String>
{
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    
    let rows = minefield.len();
    let columns = minefield[0].len();
    
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
            let left: isize = ((i % columns) as isize) - 1;
            let right: usize = (i % columns) + 1;
            let up: isize = (i as isize) - (rows as isize);
            let down: usize = i + rows;
            let leftUp: isize = (i as isize) - 1 - (rows as isize);
            let rightUp: isize = ((i + 1) as isize) - (rows as isize);
            let leftDown: isize = (i as isize) - 1 + (rows as isize);
            let rightDown: usize = i + 1 + rows;
            if left >= 0 && allInOneRow[left as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            if right < columns && allInOneRow[right] == &(42 as u8)
            {
                count = count + 1;
            }
            if up >= 0 && allInOneRow[up as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            if down < columns && allInOneRow[down] == &(42 as u8)
            {
                count = count + 1;
            }
            if left >= 0 && up >= 0 && allInOneRow[leftUp as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            if right < columns && up >= 0 && allInOneRow[rightUp as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            if left >= 0 && down < columns && allInOneRow[leftDown as usize] == &(42 as u8)
            {
                count = count + 1;
            }
            if right < columns && down < columns && allInOneRow[rightDown] == &(42 as u8)
            {
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
        println!("i={}, count={}, line: {}", i, count, line);
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
}
