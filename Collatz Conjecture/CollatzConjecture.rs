pub fn collatz(num: u64) -> Option<u64>
{
    //unimplemented!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    
    let mut n: u64 = num;
    let mut times: u64 = 0;
    let mut Answer: Option<u64> = match num
                                {
                                    0 => None,
                                    1 => Some(0),
                                    _ => Some(times),
                                };

    while n > 1 && Answer != None
    {
        println!("{}", n);
        if n % 2 == 0
        {
            Answer = match n.checked_div(2)
            {
                Some(_) =>
                        {
                            n = n/2;
                            times = times + 1;
                            Some(times)
                        },
                None => None,
            }
        }
        else if n % 2 == 1
        {
            Answer = match n.checked_add(n)
            {
                Some(_) =>
                        {
                            match n.checked_add(n + n)
                            {
                                Some(_) =>
                                        {
                                            match n.checked_add(n + n + n)
                                            {
                                                Some(_) =>
                                                        {
                                                            match n.checked_add(n + n + n + 1)
                                                            {
                                                                Some(_) =>
                                                                        {
                                                                            n = (3*n) + 1;
                                                                            times = times + 1;
                                                                            Some(times)
                                                                        }
                                                                None => None,
                                                            }
                                                        }
                                                None => None,
                                            }
                                        }
                                None => None,
                            }
                        },
                None => None,
            }
        }
    }
    Answer
}
