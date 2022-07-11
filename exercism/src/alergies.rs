pub struct Allergies {
    alergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
        let mut total = score;
        let mut result: Vec<u32> = Vec::new();
        while total > 0 {
            result.push(total % 2);
            total /= 2;
        }
        let alergies = result.iter().take(8).enumerate().filter_map(|(i, n)| match (n, i) {
            (1, 0) => Some(Allergen::Eggs),
            (1, 1) => Some(Allergen::Peanuts),
            (1, 2) => Some(Allergen::Shellfish),
            (1, 3) => Some(Allergen::Strawberries),
            (1, 4) => Some(Allergen::Tomatoes),
            (1, 5) => Some(Allergen::Chocolate),
            (1, 6) => Some(Allergen::Pollen),
            (1, 7) => Some(Allergen::Cats),
            _ => None,
        }).collect::<Vec<_>>();
        Allergies { alergies }       
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.alergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.alergies.clone()
    }
}

mod tests {
    use super::*;

    fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
        for element in expected {
            if !actual.contains(element) {
                panic!(
                    "Allergen missing\n  {:?} should be in {:?}",
                    element, actual
                );
            }
        }

        if actual.len() != expected.len() {
            panic!(
                "Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
                expected, actual
            );
        }
    }

    #[test]
    fn is_not_allergic_to_anything() {
        let allergies = Allergies::new(0);

        assert!(!allergies.is_allergic_to(&Allergen::Peanuts));

        assert!(!allergies.is_allergic_to(&Allergen::Cats));

        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }

    #[test]
    fn is_allergic_to_eggs() {
        assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
    }

    #[test]
    fn is_allergic_to_egg_shellfish_and_strawberries() {
        let allergies = Allergies::new(5);

        assert!(allergies.is_allergic_to(&Allergen::Eggs));

        assert!(allergies.is_allergic_to(&Allergen::Shellfish));

        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }

    #[test]
    fn no_allergies_at_all() {
        let expected = &[];

        let allergies = Allergies::new(0).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_just_eggs() {
        let expected = &[Allergen::Eggs];

        let allergies = Allergies::new(1).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_just_peanuts() {
        let expected = &[Allergen::Peanuts];

        let allergies = Allergies::new(2).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_just_strawberries() {
        let expected = &[Allergen::Strawberries];

        let allergies = Allergies::new(8).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_eggs_and_peanuts() {
        let expected = &[Allergen::Eggs, Allergen::Peanuts];

        let allergies = Allergies::new(3).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_eggs_and_shellfish() {
        let expected = &[Allergen::Eggs, Allergen::Shellfish];

        let allergies = Allergies::new(5).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_many_things() {
        let expected = &[
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let allergies = Allergies::new(248).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn allergic_to_everything() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let allergies = Allergies::new(255).allergies();

        compare_allergy_vectors(expected, &allergies);
    }

    #[test]
    fn scores_over_255_do_not_trigger_false_positives() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let allergies = Allergies::new(509).allergies();

        compare_allergy_vectors(expected, &allergies);
    }
}
