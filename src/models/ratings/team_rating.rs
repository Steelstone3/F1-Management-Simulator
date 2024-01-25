use crate::controller::random_generator::generate_seeded_random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeamRating {
    pub car_development: u32,
    pub car_repairs: u32,
    pub car_setup: u32,
    pub pitstops: u32,
    pub team_management: u32,
    pub overall: u32,
}
impl TeamRating {
    pub fn new(seeds: [u64; 5]) -> Self {
        let mut team_rating = Self {
            car_development: generate_seeded_random(seeds[0]),
            car_repairs: generate_seeded_random(seeds[1]),
            car_setup: generate_seeded_random(seeds[2]),
            pitstops: generate_seeded_random(seeds[3]),
            team_management: generate_seeded_random(seeds[4]),
            overall: Default::default(),
        };

        team_rating.calculate_overall_team_rating();

        team_rating
    }

    fn calculate_overall_team_rating(&mut self) {
        self.overall = (self.car_development
            + self.car_repairs
            + self.car_setup
            + self.pitstops
            + self.team_management)
            / 5
    }
}

#[cfg(test)]
mod team_rating_should {
    use super::*;

    #[test]
    fn create_a_new_team_rating() {
        // Given
        let seeds = [1, 2, 3, 4, 5];
        let expected_team_rating = TeamRating {
            car_development: 90,
            car_repairs: 64,
            car_setup: 81,
            pitstops: 84,
            team_management: 66,
            overall: 77,
        };

        // When
        let team_rating = TeamRating::new(seeds);

        // Then
        assert_eq!(expected_team_rating, team_rating);
    }

    #[test]
    fn calculate_overall_team_rating() {
        // Given
        let expected_overall_team_rating = 81;
        let mut team_rating = TeamRating {
            car_development: 76,
            car_repairs: 55,
            car_setup: 80,
            pitstops: 99,
            team_management: 99,
            overall: 0,
        };

        // When
        team_rating.calculate_overall_team_rating();

        // Then
        assert_eq!(expected_overall_team_rating, team_rating.overall);
    }
}
