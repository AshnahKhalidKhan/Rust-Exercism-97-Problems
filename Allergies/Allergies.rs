pub struct Allergies
{
    allergens: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq)]
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

impl Clone for Allergen
{
    fn clone(&self) -> Allergen
    {
        match self
        {
            Allergen::Eggs => Allergen::Eggs,
            Allergen::Peanuts => Allergen::Peanuts,
            Allergen::Shellfish => Allergen::Shellfish,
            Allergen::Strawberries => Allergen::Strawberries,
            Allergen::Tomatoes => Allergen::Tomatoes,
            Allergen::Chocolate => Allergen::Chocolate,
            Allergen::Pollen => Allergen::Pollen,
            Allergen::Cats => Allergen::Cats,
        }
    }
}

impl Allergies
{
    pub fn new(score: u32) -> Self
    {
        //unimplemented!("Given the '{score}' score, construct a new Allergies struct.");
        let mut remainingScore: u32 = score;
        let mut allergens: Vec<Allergen> = Vec::new();
        if (remainingScore >= 128)
        {
            allergens.push(Allergen::Cats);
            remainingScore = remainingScore - 128;
        }
        if (remainingScore >= 64)
        {
            allergens.push(Allergen::Pollen);
            remainingScore = remainingScore - 64;
        }
        if (remainingScore >= 32)
        {
            allergens.push(Allergen::Chocolate);
            remainingScore = remainingScore - 32;
        }
        if (remainingScore >= 16)
        {
            allergens.push(Allergen::Tomatoes);
            remainingScore = remainingScore - 16;
        }
        if (remainingScore >= 8)
        {
            allergens.push(Allergen::Strawberries);
            remainingScore = remainingScore - 8;
        }
        if (remainingScore >= 4)
        {
            allergens.push(Allergen::Shellfish);
            remainingScore = remainingScore - 4;
        }
        if (remainingScore >= 2)
        {
            allergens.push(Allergen::Peanuts);
            remainingScore = remainingScore - 2;
        }
        if (remainingScore >= 1)
        {
            allergens.push(Allergen::Eggs);
            remainingScore = remainingScore - 1;
        }
        Self
        {
            allergens
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool
    {
        //unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        if self.allergens.contains(allergen) == true
        {
            true
        }
        else
        {
            false
        }
    }

    pub fn allergies(&self) -> Vec<Allergen>
    {
        //unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
        //let mut allergicToThis: Vec<Allergen> = Vec::new();
        self.allergens.c
    }
}
