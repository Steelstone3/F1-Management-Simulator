use rand_derive2::RandGen;

use crate::controller::random_generator::generate_seeded_random;

#[derive(RandGen, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarRating {
    pub aero: u32,
    pub engine: u32,
    pub reliability: u32,
    pub tire_management: u32,
    pub overall: u32,
}

impl CarRating {
    pub fn new(seeds: [u64; 4]) -> Self {
        let mut car_rating = Self {
            aero: generate_seeded_random(seeds[0]),
            engine: generate_seeded_random(seeds[1]),
            reliability: generate_seeded_random(seeds[2]),
            tire_management: generate_seeded_random(seeds[3]),
            overall: Default::default(),
        };

        car_rating.calculate_overall_car_rating();

        car_rating
    }

    fn calculate_overall_car_rating(&mut self) {
        self.overall = (self.aero + self.engine + self.reliability + self.tire_management) / 4
    }
}

#[cfg(test)]
mod car_rating_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 2, 3, 4], CarRating {
        aero: 90,
        engine: 64,
        reliability: 81,
        tire_management: 84,
        overall: 79
    })]
    #[case([100, 200, 300, 400], CarRating {
        aero: 87,
        engine: 90,
        reliability: 57,
        tire_management: 70,
        overall: 76
    })]
    #[case([1000, 2000, 3000, 4000], CarRating {
        aero: 60,
        engine: 74,
        reliability: 75,
        tire_management: 85,
        overall: 73
    })]
    fn create_a_new_car_rating(#[case] seeds: [u64; 4], #[case] expected_car_rating: CarRating) {
        // Given
        let car_rating = CarRating::new(seeds);

        // Then
        assert_eq!(expected_car_rating, car_rating);
    }

    #[test]
    fn calculate_overall_team_rating() {
        // Given
        let expected_overall_car_rating = 77;
        let mut car_rating = CarRating {
            aero: 55,
            engine: 67,
            reliability: 88,
            tire_management: 99, 
            overall: 0,
        };

        // When
        car_rating.calculate_overall_car_rating();

        // Then
        assert_eq!(expected_overall_car_rating, car_rating.overall);
    }
}
