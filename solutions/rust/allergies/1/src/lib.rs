pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        if score < 1 {
            return Self { allergies: Vec::new() };
        }

        let allergies: Vec<Allergen> = (0..=7)
            .filter_map(|n| {
                let allergic = (score >> n) & 1;
                if allergic == 0 {
                    return None;
                }
                match (n, allergic) {
                    (0, 1) => Some(Allergen::Eggs),
                    (1, 1) => Some(Allergen::Peanuts),
                    (2, 1) => Some(Allergen::Shellfish),
                    (3, 1) => Some(Allergen::Strawberries),
                    (4, 1) => Some(Allergen::Tomatoes),
                    (5, 1) => Some(Allergen::Chocolate),
                    (6, 1) => Some(Allergen::Pollen),
                    (7, 1) => Some(Allergen::Cats),
                    _ => None,
                }
            })
            .collect();

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
