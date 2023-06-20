pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{n} make?")
    let mut melody: String = String::new();
    if (n % 3 != 0 && n % 5 != 0 && n % 7 != 0)
    {
        melody = n.to_string();
    }
    else
    {
        if (n % 3 == 0)
        {
            melody = melody + "Pling".to_string();
        }
            if (n % 5 == 0)
            {
                melody = melody + "Pling".to_string();
            }
            if (n % 7 == 0)
            {
                melody = melody + "Plong".to_string();
            }
    }

}

