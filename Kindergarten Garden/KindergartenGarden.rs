pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    //unimplemented!("Solve kindergarten-garden exercise");
    /* Grass, Clover, Radishes, Violets */
    let cupLines: Vec<_> = _diagram.lines().collect();
    let mut plants: Vec<char> = Vec::new();
    // println!("{}", _student);
    for line in cupLines
    {
        // println!("{}", line);
        let cups: Vec<_> = line.chars().collect();
        // println!("{} {}", cups[0], cups[1]);
        match _student
        {
            "Alice" =>
                    {
                        plants.push(cups[0]);
                        plants.push(cups[1]);
                    },
            "Bob" =>
                    {
                        plants.push(cups[2]);
                        plants.push(cups[3]);
                    },
            "Charlie" =>
                    {
                        plants.push(cups[4]);
                        plants.push(cups[5]);
                    },
            "David" =>
                    {
                        plants.push(cups[6]);
                        plants.push(cups[7]);
                    },
            "Eve" =>
                    {
                        plants.push(cups[8]);
                        plants.push(cups[9]);
                    },
            "Fred" =>
                    {
                        plants.push(cups[10]);
                        plants.push(cups[11]);
                    },
            "Ginny" =>
                    {
                        plants.push(cups[12]);
                        plants.push(cups[13]);
                    },
            "Harriet" =>
                    {
                        plants.push(cups[14]);
                        plants.push(cups[15]);
                    },
            "Ileana" =>
                    {
                        plants.push(cups[16]);
                        plants.push(cups[17]);
                    },
            "Joseph" =>
                    {
                        plants.push(cups[18]);
                        plants.push(cups[19]);
                    },
            "Kincaid" =>
                    {
                        plants.push(cups[20]);
                        plants.push(cups[21]);
                    },
            "Larry" =>
                    {
                        plants.push(cups[22]);
                        plants.push(cups[23]);
                    },
            _ => println!("Kuch bhi nahin"),
        }
    }
    // println!("{:?}", plants);
    let mut plantsFullNames: Vec<&str> = Vec::new();
    for plant in plants
    {
        match plant
        {
            'G' => plantsFullNames.push("grass"),
            'C' => plantsFullNames.push("clover"),
            'R' => plantsFullNames.push("radishes"),
            'V' => plantsFullNames.push("violets"),
            _ => println!("Kuch bhi nahin"),
        }
    }
    plantsFullNames
}
