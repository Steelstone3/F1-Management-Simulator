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

    pub fn calculate_season_points(&self) -> u32 {
        self.driver_1.driver_points.calculate_season_points()
            + self.driver_2.driver_points.calculate_season_points()
    }
}

#[cfg(test)]
mod team_should {
    use super::*;
    use crate::models::{drivers::driver_statistics::DriverStatistic, points::Points};

    #[test]
    fn new_team() {
        // Given
        let expected_team = Team {
            team_name: TeamName::Mercedes,
            team_statistics: TeamStatistic {
                car_development: 90,
                car_repairs: 64,
                car_setup: 81,
                pitstops: 84,
                team_management: 66,
                overall: 77,
            },
            car: Car {
                aero: 56,
                engine: 89,
                reliability: 68,
                tire_management: 97,
                overall: 77,
            },
            driver_1: Driver {
                driver_name: DriverName::LewisHamilton,
                driver_statistics: DriverStatistic {
                    awareness: 87,
                    consistency: 90,
                    experience: 57,
                    race_craft: 70,
                    pace: 50,
                    overall: 70,
                },
                driver_points: Points::default(),
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                driver_statistics: DriverStatistic {
                    awareness: 60,
                    consistency: 74,
                    experience: 75,
                    race_craft: 85,
                    pace: 94,
                    overall: 77,
                },
                driver_points: Points::default(),
            },
        };

        // When
        let team = Team::new(
            TeamName::Mercedes,
            DriverName::LewisHamilton,
            DriverName::GeorgeRussell,
            [1, 2, 3, 4, 5],
            [10, 20, 30, 40],
            [100, 200, 300, 400, 500],
            [1000, 2000, 3000, 4000, 5000],
        );

        // Then
        assert_eq!(expected_team, team);
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
