pub fn collatz(n: u64) -> Option<u64> {
    //unimplemented!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut n: u64 = n;
    let mut Answer: Option<u64> = Some(0);

    // if n % 2 == 0
    // {
    //     n = n/2;
    // }
    // else if n % 2 == 1
    // {
    //     n = (3*n) + 1;
    // }

    let mut times: u64 = 0;
    while n != 1 && Answer != None
    {
        if n % 2 == 0
        {
            match n.checked_div(2)
            {
                Some(_) =>
                {
                    n = n/2;
                    times = times + 1;
                    Answer = Some(times);
                },
                None => Answer = None,
            }
        }
        else if n % 2 == 1
        {
            match n.checked_add(n+n+n+1)
            {
                Some(_) =>
                {
                    n = 3*n + 1;
                    times = times + 1;
                    Answer = Some(times);
                },
                None => Answer = None,
            }
        }
    }
    Answer
}
