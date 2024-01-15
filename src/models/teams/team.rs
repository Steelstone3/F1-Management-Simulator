use std::fmt::Display;

use super::{team_name::TeamName, team_seeds::TeamSeed, team_statistics::TeamStatistic};
use crate::models::{
    car::Car,
    drivers::{driver::Driver, driver_name::DriverName},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
        team_seeds: TeamSeed,
    ) -> Self {
        Self {
            team_name,
            team_statistics: TeamStatistic::new(team_seeds.team_statistics_seeds),
            car: Car::new(team_seeds.car_seeds),
            driver_1: Driver::new(driver_name_1, team_name, team_seeds.driver_1_seeds),
            driver_2: Driver::new(driver_name_2, team_name, team_seeds.driver_2_seeds),
        }
    }

    pub fn calculate_drivers_overall_race_chance(&mut self) {
        let team_chance = self.team_statistics.overall + self.car.overall;

        self.driver_1.overall_race_chance =
            (team_chance + self.driver_1.driver_statistics.overall) / 3;

        self.driver_2.overall_race_chance =
            (team_chance + self.driver_2.driver_statistics.overall) / 3;
    }

    pub fn add_race_points(&mut self, driver: Driver) {
        if self.driver_1.driver_name == driver.driver_name {
            self.driver_1.driver_race_points.race_points = driver.driver_race_points.race_points;
        } else if self.driver_2.driver_name == driver.driver_name {
            self.driver_2.driver_race_points.race_points = driver.driver_race_points.race_points;
        } else {
            panic!("No driver in the team to allocate points")
        }
    }

    pub fn calculate_season_points(&self) -> u32 {
        self.driver_1.driver_season_points.season_points
            + self.driver_2.driver_season_points.season_points
    }
}

impl Display for Team {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatting,
            "| {} | {} Points | {} | {} Points | {} | {} Points |",
            self.team_name,
            self.calculate_season_points(),
            self.driver_1.driver_name,
            self.driver_1.driver_season_points.season_points,
            self.driver_2.driver_name,
            self.driver_2.driver_season_points.season_points
        )
    }
}

#[cfg(test)]
mod team_should {
    use crate::models::{
        drivers::driver_statistics::DriverStatistic, races::race_points::RacePoints,
        seasons::season_points::SeasonPoints,
    };
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(DriverName::LewisHamilton, 15)]
    #[case(DriverName::LewisHamilton, 12)]
    #[case(DriverName::LewisHamilton, 25)]
    #[case(DriverName::LewisHamilton, 2)]
    #[case(DriverName::LewisHamilton, 0)]
    #[case(DriverName::GeorgeRussell, 15)]
    #[case(DriverName::GeorgeRussell, 12)]
    #[case(DriverName::GeorgeRussell, 25)]
    #[case(DriverName::GeorgeRussell, 2)]
    #[case(DriverName::GeorgeRussell, 0)]
    fn add_points_from_driver(#[case] driver_name: DriverName, #[case] race_points: u32) {
        // Given
        let driver = Driver {
            driver_name,
            team_name: TeamName::Mercedes,
            driver_race_points: RacePoints { race_points },
            ..Default::default()
        };
        let mut team = Team {
            team_name: TeamName::Mercedes,
            driver_1: Driver {
                driver_name: DriverName::LewisHamilton,
                team_name: TeamName::Mercedes,
                driver_race_points: RacePoints {
                    race_points: Default::default(),
                },
                ..Default::default()
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                team_name: TeamName::Mercedes,
                driver_race_points: RacePoints {
                    race_points: Default::default(),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        team.add_race_points(driver);

        // Then
        assert_eq!(race_points, driver.driver_race_points.race_points);
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
                driver_season_points: SeasonPoints::default(),
                driver_race_points: RacePoints::default(),
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
                driver_season_points: SeasonPoints::default(),
                driver_race_points: RacePoints::default(),
                overall_race_chance: Default::default(),
            },
        };

        // When
        let team = Team::new(
            TeamName::Mercedes,
            DriverName::LewisHamilton,
            DriverName::GeorgeRussell,
            TeamSeed {
                team_statistics_seeds: [1, 2, 3, 4, 5],
                car_seeds: [10, 20, 30, 40],
                driver_1_seeds: [100, 200, 300, 400, 500],
                driver_2_seeds: [1000, 2000, 3000, 4000, 5000],
            },
        );

        // Then
        assert_eq!(expected_team, team);
    }

    #[test]
    fn display_the_team() {
        // Given
        let expected_team_display = "Team: Mercedes\nTeam Points: 427\n\nDriver: Lewis Hamilton\nPoints: 236\n\nDriver: George Russell\nPoints: 191\n\n\n";
        let team = Team {
            team_name: TeamName::Mercedes,
            driver_1: Driver {
                driver_name: DriverName::LewisHamilton,
                driver_season_points: SeasonPoints { season_points: 236 },
                ..Default::default()
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                driver_season_points: SeasonPoints { season_points: 191 },
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let team_display = team.to_string();

        // Then
        assert_eq!(expected_team_display, team_display)
    }

    #[test]
    fn calculate_the_drivers_overall_race_chance() {
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
    #[case(232, 111, 343)]
    #[case(45, 10, 55)]
    fn calculate_team_season_points(
        #[case] driver_1_race_points: u32,
        #[case] driver_2_race_points: u32,
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
                driver_season_points: SeasonPoints {
                    season_points: driver_1_race_points,
                },
                ..Default::default()
            },
            driver_2: Driver {
                driver_season_points: SeasonPoints {
                    season_points: driver_2_race_points,
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
