use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::{
    controller::random_generator::generate_seed,
    models::{
        car::Car,
        drivers::{driver::Driver, driver_name::DriverName},
    },
};
use rand::random;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub team_statistics: TeamStatistic,
    pub car: Car,
    pub driver_1: Driver,
    pub driver_2: Driver,
}

impl Team {
    pub fn new(
        driver_names: [DriverName; 2],
        team_statistics_seeds: [u64; 5],
        car_seeds: [u64; 4],
    ) -> Self {
        Self {
            name: random(),
            team_statistics: TeamStatistic::new(team_statistics_seeds),
            car: Car::new(car_seeds),
            driver_1: Driver::new(driver_names[0]),
            driver_2: Driver::new(driver_names[1]),
        }
    }

    pub fn calculate_driver_1_overall(&self) -> u32 {
        (self.team_statistics.overall + self.car.overall + self.driver_1.statistics.overall) / 3
    }

    pub fn calculate_driver_2_overall(&self) -> u32 {
        (self.team_statistics.overall + self.car.overall + self.driver_2.statistics.overall) / 3
    }

    pub fn calculate_season_points(&self) -> u32 {
        self.driver_1.points.calculate_season_points()
            + self.driver_2.points.calculate_season_points()
    }
}

#[cfg(test)]
mod team_should {
    use crate::models::{drivers::driver_statistics::DriverStatistic, points::Points};

    use super::*;

    #[test]
    fn calculate_driver_1_overall() {
        // Given
        let expected_overall = 73;
        let team = Team {
            team_statistics: TeamStatistic {
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
            team_statistics: TeamStatistic {
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

    #[test]
    fn calculate_team_season_points() {
        // Given
        let expected_season_points = 349;
        let team = Team {
            team_statistics: TeamStatistic {
                ..Default::default()
            },
            car: Car {
                ..Default::default()
            },
            driver_1: Driver {
                points: Points::new([25, 18, 15, 15, 25, 18, 18, 25, 15, 25]),
                ..Default::default()
            },
            driver_2: Driver {
                points: Points::new([1, 25, 18, 25, 18, 15, 6, 2, 15, 25]),
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let season_points = team.calculate_season_points();

        // Then
        assert_eq!(expected_season_points, season_points)
    }
}
