use crate::controller::random_generator::generate_seeded_random;
use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TeamStatistic {
    pub car_development: u32,
    pub car_repairs: u32,
    pub car_setup: u32,
    pub pitstops: u32,
    pub team_management: u32,
    pub overall: u32,
}

impl TeamStatistic {
    pub fn new(seeds: [u64; 5]) -> Self {
        let car_development = generate_seeded_random(seeds[0]);
        let car_repairs = generate_seeded_random(seeds[1]);
        let car_setup = generate_seeded_random(seeds[2]);
        let pitstops = generate_seeded_random(seeds[3]);
        let team_management = generate_seeded_random(seeds[4]);

        let overall = (car_development + car_repairs + car_setup + pitstops + team_management) / 5;

        Self {
            car_development,
            car_repairs,
            car_setup,
            pitstops,
            team_management,
            overall,
        }
    }
}

impl Display for TeamStatistic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Car Development: {}\nCar Repairs: {}\nCar Setup: {}\nPitstops: {}\nTeam Management: {}\nOverall: {}",
            self.car_development,
            self.car_repairs,
            self.car_setup,
            self.pitstops,
            self.team_management,
            self.overall
        )
    }
}

#[cfg(test)]
mod driver_statistics_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 2, 3, 4, 5], TeamStatistic {
        car_development: 90,
        car_repairs: 64,
        car_setup: 81,
        pitstops: 84,
        team_management: 66,
        overall: 77,
    })]
    #[case([100, 200, 300, 400, 500], TeamStatistic {
        car_development: 87,
        car_repairs: 90,
        car_setup: 57,
        pitstops: 70,
        team_management: 50,
        overall: 70,
    })]
    #[case([1000, 2000, 3000, 4000, 5000], TeamStatistic {
        car_development: 60,
        car_repairs: 74,
        car_setup: 75,
        pitstops: 85,
        team_management: 94,
        overall: 77,
    })]
    fn new_driver_statistics(#[case] seeds: [u64; 5], #[case] expected: TeamStatistic) {
        // When
        let driver_statistics = TeamStatistic::new(seeds);

        // Then
        assert_eq!(expected, driver_statistics);
    }

    #[test]
    fn display() {
        // Given
        let expected =
            "Car Development: 90\nCar Repairs: 64\nCar Setup: 81\nPitstops: 84\nTeam Management: 66\nOverall: 77"
                .to_string();

        // When
        let display = TeamStatistic::new([1, 2, 3, 4, 5]).to_string();

        // Then
        assert_eq!(expected, display);
    }
}
