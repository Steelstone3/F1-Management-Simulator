use crate::controller::random_generator::generate_seeded_random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriverRating {
    pub awareness: u32,
    pub consistency: u32,
    pub experience: u32,
    pub race_craft: u32,
    pub pace: u32,
    pub overall: u32,
    pub overall_race_chance: u32,
}

impl DriverRating {
    pub fn new(seeds: [u64; 5]) -> Self {
        let mut driver_rating = Self {
            awareness: generate_seeded_random(seeds[0]),
            consistency: generate_seeded_random(seeds[1]),
            experience: generate_seeded_random(seeds[2]),
            race_craft: generate_seeded_random(seeds[3]),
            pace: generate_seeded_random(seeds[4]),
            overall: Default::default(),
            overall_race_chance: Default::default(),
        };

        driver_rating.calculate_overall_driver_rating();

        driver_rating
    }

    fn calculate_overall_driver_rating(&mut self) {
        self.overall = (self.awareness
            + self.consistency
            + self.experience
            + self.race_craft
            + self.pace)
            / 5
    }
}

#[cfg(test)]
mod driver_rating_should {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case([1, 2, 3, 4, 5], DriverRating {
        awareness: 90,
        consistency: 64,
        experience: 81,
        race_craft: 84,
        pace: 66,
        overall: 77, 
        overall_race_chance: Default::default(),
    })]
    #[case([100, 200, 300, 400, 500], DriverRating {
        awareness: 87,
        consistency: 90,
        experience: 57,
        race_craft: 70,
        pace: 50,
        overall: 70, 
        overall_race_chance: Default::default(),
    })]
    #[case([1000, 2000, 3000, 4000, 5000], DriverRating {
        awareness: 60,
        consistency: 74,
        experience: 75,
        race_craft: 85,
        pace: 94,
        overall: 77, 
        overall_race_chance: Default::default(),
    })]
    fn create_a_new_car_rating(#[case] seeds: [u64; 5], #[case] expected_driver_rating: DriverRating) {
        // Given
        let driver_rating = DriverRating::new(seeds);

        // Then
        assert_eq!(expected_driver_rating, driver_rating);
    }

    #[test]
    fn calculate_overall_team_rating() {
        // Given
        let expected_overall_driver_rating = 81;
        let mut driver_rating = DriverRating {
            awareness: 55,
            consistency: 76,
            experience: 90,
            race_craft: 99,
            pace: 87,
            overall: 77, 
            overall_race_chance: Default::default(),
        };

        // When
        driver_rating.calculate_overall_driver_rating();

        // Then
        assert_eq!(expected_overall_driver_rating, driver_rating.overall);
    }
}
