pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    //unimplemented!("Solve kindergarten-garden exercise");
    /* Grass, Clover, Radishes, Violets */
    let cupLines: Vec<_> = _diagram.lines().collect();
    let mut plants: Vec<char> = Vec::new();
    println!("{}", _student);
    for line in cupLines
    {
        println!("{}", line);
        let cups: Vec<_> = line.chars().collect();
        println!("{} {}", cups[0], cups[1]);
        match _student
        {
            "Alice" =>
                    {
                        plants.push(cups[0]); //expected `String`, found `char` so use the damn .to_string() function
                        plants.push(cups[1]);
                    },
            _ => println!("Kuch bhi nahin"),
        }
        // if _student == "Alice"
        // {
        //     plants.push(cups[0].to_string());
        //     plants.push(cups[1].to_string());
        // }
    }
    println!("{:?}", plants);
    let mut plantsFullNames: Vec<&str> = Vec::new();
    for plant in plants
    {
        match plant
        {
            'G' => plantsFullNames.push("grass"),//.to_string()), //expected `String`, found `char` so use the damn .to_string() function
            'C' => plantsFullNames.push("clover"),//.to_string()),
            'R' => plantsFullNames.push("radishes"),//.to_string()),
            'V' => plantsFullNames.push("violets"),//.to_string()),
            _ => println!("Kuch bhi nahin"),
        }
    }
    plantsFullNames
}
