use std::fmt::Display;

use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::models::{
    car::Car,
    drivers::{
        driver::{self, Driver},
        driver_name::DriverName,
    },
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
            driver_1: Driver::new(driver_name_1, team_name, driver_1_seeds),
            driver_2: Driver::new(driver_name_2, team_name, driver_2_seeds),
        }
    }

    pub fn calculate_drivers_overall_race_chance(&mut self) {
        let team_chance = self.team_statistics.overall + self.car.overall;

        self.driver_1.overall_race_chance =
            (team_chance + self.driver_1.driver_statistics.overall) / 3;

        self.driver_2.overall_race_chance =
            (team_chance + self.driver_2.driver_statistics.overall) / 3;
    }

    pub fn add_points(&mut self, driver: Driver, race_number: usize) {
        if self.driver_1.driver_name == driver.driver_name {
            self.driver_1.driver_points.race_points[race_number] =
                driver.driver_points.race_points[race_number];
        } else if self.driver_2.driver_name == driver.driver_name {
            self.driver_2.driver_points.race_points[race_number] =
                driver.driver_points.race_points[race_number];
        } else {
            panic!("No driver in the team to allocate points")
        }
    }

    pub fn calculate_season_points(&self) -> u32 {
        self.driver_1.driver_points.calculate_season_points()
            + self.driver_2.driver_points.calculate_season_points()
    }
}

// TODO add a display that shows team name and points
impl Display for Team {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

#[cfg(test)]
mod team_should {
    use rstest::rstest;

    use super::*;
    use crate::models::{
        drivers::driver_statistics::DriverStatistic, points::Points,
        season::NUMBER_OF_RACES_IN_A_SEASON,
    };

    #[rstest]
    #[case(2, 15)]
    fn add_points_from_driver(#[case] race_number: usize, #[case] expected_race_points: u32) {
        // Given
        let driver = Driver {
            driver_name: DriverName::LewisHamilton,
            team_name: TeamName::Mercedes,
            driver_points: Points {
                race_points: [25, 18, 15, 12, 10, 8, 6, 4, 2, 0],
            },
            ..Default::default()
        };
        let mut team = Team {
            team_name: TeamName::Mercedes,
            driver_1: Driver {
                driver_name: DriverName::LewisHamilton,
                team_name: TeamName::Mercedes,
                driver_points: Points {
                    race_points: Default::default(),
                },
                ..Default::default()
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                team_name: TeamName::Mercedes,
                driver_points: Points {
                    race_points: Default::default(),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        team.add_points(driver, race_number);

        // Then
        assert_eq!(
            expected_race_points,
            driver.driver_points.race_points[race_number]
        );
    }

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
                team_name: TeamName::Mercedes,
                driver_statistics: DriverStatistic {
                    awareness: 87,
                    consistency: 90,
                    experience: 57,
                    race_craft: 70,
                    pace: 50,
                    overall: 70,
                },
                driver_points: Points::default(),
                overall_race_chance: Default::default(),
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                team_name: TeamName::Mercedes,
                driver_statistics: DriverStatistic {
                    awareness: 60,
                    consistency: 74,
                    experience: 75,
                    race_craft: 85,
                    pace: 94,
                    overall: 77,
                },
                driver_points: Points::default(),
                overall_race_chance: Default::default(),
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
    fn calculate_drivers_overall_race_chance() {
        // Given
        let driver_1_expected_overall = 80;
        let driver_2_expected_overall = 69;
        let mut team = Team {
            team_statistics: TeamStatistic {
                overall: 65,
                ..Default::default()
            },
            car: Car {
                overall: 87,
                ..Default::default()
            },
            driver_1: Driver {
                driver_statistics: DriverStatistic {
                    overall: 89,
                    ..Default::default()
                },
                ..Default::default()
            },
            driver_2: Driver {
                driver_statistics: DriverStatistic {
                    overall: 55,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        team.calculate_drivers_overall_race_chance();

        // Then
        assert_eq!(driver_1_expected_overall, team.driver_1.overall_race_chance);
        assert_eq!(driver_2_expected_overall, team.driver_2.overall_race_chance);
    }

    #[rstest]
    #[case([25, 18, 15, 15, 25, 18, 18, 25, 15, 25],[1, 25, 18, 25, 18, 15, 6, 2, 15, 25],349)]
    #[case([10,8,6,4,15,18,4,6,8,2],[1,1,1,4,6,8,12,10,12,15],151)]
    #[case([10,10,10,10,10,10,10,10,10,10],[10,10,10,10,10,10,10,10,10,10],200)]
    fn calculate_team_season_points(
        #[case] driver_1_race_points: [u32; NUMBER_OF_RACES_IN_A_SEASON],
        #[case] driver_2_race_points: [u32; NUMBER_OF_RACES_IN_A_SEASON],
        #[case] expected_season_points: u32,
    ) {
        // Given
        let team = Team {
            team_statistics: TeamStatistic {
                ..Default::default()
            },
            car: Car {
                ..Default::default()
            },
            driver_1: Driver {
                driver_points: Points {
                    race_points: driver_1_race_points,
                },
                ..Default::default()
            },
            driver_2: Driver {
                driver_points: Points {
                    race_points: driver_2_race_points,
                },
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
