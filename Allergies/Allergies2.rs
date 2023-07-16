use std::collections::HashMap;

pub struct Allergies
{
    stuffAllergicTo: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        let scoreToAllergen: HashMap<u32, Allergen> = HashMap::from([
            (1, Allergen::Eggs),
            (2, Allergen::Peanuts),
            (4, Allergen::Shellfish),
            (8, Allergen::Strawberries),
            (16, Allergen::Tomatoes),
            (32, Allergen::Chocolate),
            (64, Allergen::Pollen),
            (128, Allergen::Cats),
        ]);
        
        let mut newStuffAllergicTo: Vec<Allergen> = Vec::new();
        let mut power: u32 = 7;
        let Two: u32 = 2;
        while power >= 0
        {
            if remainingScore >= Two.pow(power)
            {
                newStuffAllergicTo.push(*scoreToAllergen.get(&Two.pow(power)).unwrap());
                remainingScore = remainingScore - Two.pow(power);
            }   
            match power
            {
                0 => break,
                _ => power = power - 1
            };
        }
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
