pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    //unimplemented!("Solve kindergarten-garden exercise");
    /* Grass, Clover, Radishes, Violets */
    let cupLines: Vec<_> = _diagram.split("/n").collect();
    let mut plants: Vec<String> = Vec::new();
    for line in cupLines
    {
        let cups: Vec<_> = line.chars().collect();
        match _student
        {
            "Alice" =>
                    {
                        plants.push(cups[0].to_string()); //expected `String`, found `char` so use the damn .to_string() function
                        plants.push(cups[1].to_string());
                    },
            _ => println!("Kuch bhi nahin"),
        }
    }
    let mut plantsFullNames: Vec<&str> = Vec::new();
    for plant in plants
    {
        match _student
        {
            "G" => plantsFullNames.push("grass"),//.to_string()), //expected `String`, found `char` so use the damn .to_string() function
            "C" => plantsFullNames.push("clover"),//.to_string()),
            "R" => plantsFullNames.push("radishes"),//.to_string()),
            "V" => plantsFullNames.push("violets"),//.to_string()),
            _ => println!("Kuch bhi nahin"),
        }
    }
    plantsFullNames
}
