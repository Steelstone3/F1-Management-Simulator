use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::{
    controller::random_generator::generate_seed,
    models::{car::Car, drivers::driver::Driver},
};
use rand::random;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub statistics: TeamStatistic,
    pub car: Car,
    pub driver_1: Driver,
    pub driver_2: Driver,
}

impl Team {
    pub fn new() -> Self {
        let team_statistics_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];
        let car_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];

        Self {
            name: random(),
            statistics: TeamStatistic::new(team_statistics_seeds),
            car: Car::new(car_seeds),
            driver_1: Driver::new(),
            driver_2: Driver::new(),
        }
    }

    pub fn calculate_driver_1_overall(&self) -> u32 {
        (self.statistics.overall + self.car.overall + self.driver_1.statistics.overall) / 3
    }

    pub fn calculate_driver_2_overall(&self) -> u32 {
        (self.statistics.overall + self.car.overall + self.driver_2.statistics.overall) / 3
    }
}

#[cfg(test)]
mod team_should {
    use crate::models::drivers::driver_statistics::DriverStatistic;

    use super::*;

    #[test]
    fn calculate_driver_1_overall() {
        // Given
        let expected_overall = 73;
        let team = Team {
            statistics: TeamStatistic {
                overall: 55,
                ..Default::default()
            },
            car: Car {
                overall: 77,
                ..Default::default()
            },
            driver_1: Driver {
                statistics: DriverStatistic {
                    overall: 89,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let overall = team.calculate_driver_1_overall();

        // Then
        assert_eq!(expected_overall, overall)
    }

    #[test]
    fn calculate_driver_2_overall() {
        // Given
        let expected_overall = 80;
        let team = Team {
            statistics: TeamStatistic {
                overall: 65,
                ..Default::default()
            },
            car: Car {
                overall: 87,
                ..Default::default()
            },
            driver_2: Driver {
                statistics: DriverStatistic {
                    overall: 89,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let overall = team.calculate_driver_2_overall();

        // Then
        assert_eq!(expected_overall, overall)
    }
}
