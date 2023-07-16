use std::collections::HashMap;

pub struct Allergies
{
    stuffAllergicTo: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen
{
    Eggs, // 1
    Peanuts, //2
    Shellfish, //4
    Strawberries, //8
    Tomatoes, //16
    Chocolate, //32
    Pollen, //64
    Cats, //128
}

impl Allergies
{
    pub fn new(score: u32) -> Self
    {
        //unimplemented!("Given the '{score}' score, construct a new Allergies struct.");
        let mut remainingScore: u32 = match score > 255
        {
            true => score - 256,
            false => score
        };
        let powerOfTwoToAllergen: HashMap<u32, Allergen> = HashMap::from([
            (1, Allergen::Eggs),
            (2, Allergen::Peanuts),
            (4, Allergen::Shellfish),
            (8, Allergen::Strawberries),
            (16, Allergen::Tomatoes),
            (32, Allergen::Chocolate),
            (64, Allergen::Pollen),
            (128, Allergen::Cats),
        ]);
        //7 allergies....
        let mut newStuffAllergicTo: Vec<Allergen> = Vec::new();
        let mut power: u32 = 7;
        let Two: u32 = 2;
        while power >= 0
        {
            println!("{} {}", power, Two.pow(power));
            if remainingScore >= Two.pow(power)
            {
                
            }
            power = power - 1;
        }
        // while power >= 0
        // {
        //     match remainingScore >= (2 as u32).pow(power)
        //     {
        //         true =>
        //         {
        //             newStuffAllergicTo.push(powerOfTwoToAllergen.get(&(2 as u32).pow(power)).unwrap().clone());
        //             remainingScore = remainingScore - (2 as u32).pow(power);
        //         },
        //         false => power = power - 1,
        //     };
        //     power = power - 1;
        // }
        // let mut stuffAllergicTo: Vec<Allergen> = Vec::new();
        // if (remainingScore >= 128)
        // {
        //     stuffAllergicTo.push(Allergen::Cats);
        //     remainingScore = remainingScore - 128;
        // }
        // if (remainingScore >= 64)
        // {
        //     stuffAllergicTo.push(Allergen::Pollen);
        //     remainingScore = remainingScore - 64;
        // }
        // if (remainingScore >= 32)
        // {
        //     stuffAllergicTo.push(Allergen::Chocolate);
        //     remainingScore = remainingScore - 32;
        // }
        // if (remainingScore >= 16)
        // {
        //     stuffAllergicTo.push(Allergen::Tomatoes);
        //     remainingScore = remainingScore - 16;
        // }
        // if (remainingScore >= 8)
        // {
        //     stuffAllergicTo.push(Allergen::Strawberries);
        //     remainingScore = remainingScore - 8;
        // }
        // if (remainingScore >= 4)
        // {
        //     stuffAllergicTo.push(Allergen::Shellfish);
        //     remainingScore = remainingScore - 4;
        // }
        // if (remainingScore >= 2)
        // {
        //     stuffAllergicTo.push(Allergen::Peanuts);
        //     remainingScore = remainingScore - 2;
        // }
        // if (remainingScore >= 1)
        // {
        //     stuffAllergicTo.push(Allergen::Eggs);
        //     remainingScore = remainingScore - 1;
        // }
        // Self
        // {
        //     stuffAllergicTo
        // }
        Self
        {
            stuffAllergicTo: newStuffAllergicTo
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool
    {
        //unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        match self.stuffAllergicTo.contains(allergen)
        {
            true => true,
            false => false
        }
    }

    pub fn allergies(&self) -> Vec<Allergen>
    {
        //unimplemented!("Return the list of stuffAllergicTo contained within the score with which the Allergies struct was made.");
        self.stuffAllergicTo.clone()
    }
}
