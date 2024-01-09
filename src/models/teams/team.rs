use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::models::{
    car::Car,
    drivers::{driver::Driver, driver_name::DriverName},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Team {
    pub team_name: TeamName,
    pub team_statistics: TeamStatistic,
    pub car: Car,
    pub driver_1: Driver,
    pub driver_2: Driver,
}

impl Team {
    pub fn new(
        team_name: TeamName,
        driver_name_1: DriverName,
        driver_name_2: DriverName,
        team_statistics_seeds: [u64; 5],
        car_seeds: [u64; 4],
        driver_1_seeds: [u64; 5],
        driver_2_seeds: [u64; 5],
    ) -> Self {
        Self {
            team_name,
            team_statistics: TeamStatistic::new(team_statistics_seeds),
            car: Car::new(car_seeds),
            driver_1: Driver::new(driver_name_1, driver_1_seeds),
            driver_2: Driver::new(driver_name_2, driver_2_seeds),
        }
    }

    pub fn calculate_driver_1_overall(&self) -> u32 {
        (self.team_statistics.overall + self.car.overall + self.driver_1.driver_statistics.overall) / 3
    }

    pub fn calculate_driver_2_overall(&self) -> u32 {
        (self.team_statistics.overall + self.car.overall + self.driver_2.driver_statistics.overall) / 3
    }

    pub fn calculate_season_points(&self) -> u32 {
        self.driver_1.driver_points.calculate_season_points()
            + self.driver_2.driver_points.calculate_season_points()
    }
}

#[cfg(test)]
mod team_should {
    use super::*;
    use crate::{
        controller::random_generator::{generate_4_seeds, generate_5_seeds},
        models::{drivers::driver_statistics::DriverStatistic, points::Points},
    };

    #[test]
    fn new_team() {
        // Given
        let team = Team::new(
            TeamName::Mercedes,
            DriverName::LewisHamilton,
            DriverName::GeorgeRussell,
            generate_5_seeds(),
            generate_4_seeds(),
            generate_5_seeds(),
            generate_5_seeds(),
        );

        // Then
        assert_eq!(TeamName::Mercedes, team.team_name);
        // assert_eq!(, team.driver_1);
        // assert_eq!(, team.driver_2);
    }

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
                driver_statistics: DriverStatistic {
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
                driver_statistics: DriverStatistic {
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
                driver_points: Points::new([25, 18, 15, 15, 25, 18, 18, 25, 15, 25]),
                ..Default::default()
            },
            driver_2: Driver {
                driver_points: Points::new([1, 25, 18, 25, 18, 15, 6, 2, 15, 25]),
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
