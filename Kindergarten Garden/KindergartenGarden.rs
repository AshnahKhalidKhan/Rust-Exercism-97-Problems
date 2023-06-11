pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    //unimplemented!("Solve kindergarten-garden exercise");
    /* Grass, Clover, Radishes, Violets */
    let cupLines: Vec<_> = _diagram.split("\n").collect();
    let mut plants: Vec<String> = Vec::new();
    for line in cuplines
    {
        let cups: Vec<_> = line.chars().collect();
        match _student
        {
            "Alice" =>
                    {
                        plants.push(cups[0]);
                        plants.push(cups[1]);
                    },
            _ => println("Kuch bhi nahin"),
        }
    }
    let mut plantsFullNames: Vec<String> = Vec::new();
    for plant in plants
    {
        match _student
        {
            "G" => plantsFullNames.push("grass"),
            "C" => plantsFullNames.push("clover"),
            "R" => plantsFullNames.push("radishes"),
            "V" => plantsFullNames.push("violets"),
            _ => println("Kuch bhi nahin"),
        }
    }
}
