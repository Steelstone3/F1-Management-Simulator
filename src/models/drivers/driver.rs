use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use crate::models::{points::Points, teams::{team::Team, team_name::{TeamName, self}}};
use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_statistics: DriverStatistic,
    pub driver_points: Points,
}

impl Driver {
    pub fn new(driver_name: DriverName, team_name:TeamName, driver_statistics_seeds: [u64; 5]) -> Self {
        Self {
            driver_name,
            team_name,
            driver_statistics: DriverStatistic::new(driver_statistics_seeds),
            driver_points: Default::default(),
        }
    }

    pub fn calculate_driver_overall(&self, team: &Team) -> u32 {
        (team.team_statistics.overall + team.car.overall + self.driver_statistics.overall)
            / 3
    }
}

impl Display for Driver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}\n{}\n\n\n",
            self.driver_name, self.driver_statistics
        )
    }
}

#[cfg(test)]
mod driver_should {
    use crate::models::{
        drivers::{driver::Driver, driver_name::DriverName, driver_statistics::DriverStatistic},
        points::Points, teams::{team::Team, team_statistics::TeamStatistic, team_name::TeamName}, car::Car,
    };

    #[test]
    fn new_driver() {
        // Given
        let expected_driver = Driver {
            driver_name: DriverName::LewisHamilton,
            team_name: TeamName::Mercedes,
            driver_statistics: DriverStatistic {
                awareness: 90,
                consistency: 64,
                experience: 81,
                race_craft: 84,
                pace: 66,
                overall: 77,
            },
            driver_points: Points::default(),
        };

        // When
        let driver = Driver::new(DriverName::LewisHamilton, TeamName::Mercedes, [1, 2, 3, 4, 5]);

        // Then
        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn display_driver() {
        // Given
        let expected_driver_display = "Lewis Hamilton\nAwareness: 90\nConsistency: 64\nExpierence: 81\nPace: 66\nRace Craft: 84\nOverall: 77\n\n\n".to_string();
        let driver = Driver {
            driver_name: DriverName::LewisHamilton,
            team_name: TeamName::Mercedes,
            driver_statistics: DriverStatistic {
                awareness: 90,
                consistency: 64,
                experience: 81,
                race_craft: 84,
                pace: 66,
                overall: 77,
            },
            driver_points: Points::default(),
        };

        // When
        let driver_display = driver.to_string();

        // Then
        assert_eq!(expected_driver_display, driver_display);
    }

    #[test]
    fn calculate_driver_overall() {
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
        let overall = team.driver_2.calculate_driver_overall(&team);

        // Then
        assert_eq!(expected_overall, overall)
    }
}
