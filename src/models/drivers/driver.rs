use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use crate::models::{
    races::{race_grid::TEAMS_ON_THE_RACE_GRID, race_points::RacePoints},
    seasons::season_points::SeasonPoints,
    teams::{team::Team, team_name::TeamName},
};
use core::panic;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_statistics: DriverStatistic,
    pub driver_race_points: RacePoints,
    pub driver_season_points: SeasonPoints,
    pub overall_race_chance: u32,
}

impl Driver {
    pub fn new(
        driver_name: DriverName,
        team_name: TeamName,
        driver_statistics_seeds: [u64; 5],
    ) -> Self {
        Self {
            driver_name,
            team_name,
            driver_statistics: DriverStatistic::new(driver_statistics_seeds),
            driver_season_points: Default::default(),
            driver_race_points: Default::default(),
            overall_race_chance: Default::default(),
        }
    }

    pub fn find_team(&self, teams: &[Team; TEAMS_ON_THE_RACE_GRID]) -> usize {
        let mut team_index = 0;

        for team in teams {
            if self.team_name == team.team_name {
                return team_index;
            }
            team_index += 1;
        }

        panic!("No team found for driver");
    }
}

impl Display for Driver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Driver: {}\nTeam: {}\nDriver Points: {}\n",
            self.driver_name, self.team_name, self.driver_race_points.race_points
        )
    }
}

#[cfg(test)]
mod driver_should {
    use rstest::rstest;

    use crate::models::{
        drivers::{driver::Driver, driver_name::DriverName, driver_statistics::DriverStatistic},
        races::{race_grid::TEAMS_ON_THE_RACE_GRID, race_points::RacePoints},
        seasons::season_points::SeasonPoints,
        teams::{team::Team, team_name::TeamName},
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
            driver_season_points: SeasonPoints::default(),
            driver_race_points: RacePoints::default(),
            overall_race_chance: Default::default(),
        };

        // When
        let driver = Driver::new(
            DriverName::LewisHamilton,
            TeamName::Mercedes,
            [1, 2, 3, 4, 5],
        );

        // Then
        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn display_driver() {
        // Given
        let expected_driver_display =
            "Driver: Lewis Hamilton\nTeam: Mercedes\nDriver Points: 0\n".to_string();
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
            driver_season_points: SeasonPoints::default(),
            driver_race_points: RacePoints::default(),
            overall_race_chance: Default::default(),
        };

        // When
        let driver_display = driver.to_string();

        // Then
        assert_eq!(expected_driver_display, driver_display);
    }

    #[rstest]
    #[case(2, TeamName::Mercedes)]
    #[case(7, TeamName::AlphaTauri)]
    #[case(0, TeamName::RedBull)]
    fn find_team_associated_with_driver(
        #[case] expected_team_index: usize,
        #[case] team_name: TeamName,
    ) {
        // Given
        let driver = Driver {
            team_name,
            ..Default::default()
        };
        let teams: [Team; TEAMS_ON_THE_RACE_GRID] = [
            Team {
                team_name: TeamName::RedBull,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Ferrari,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Mercedes,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Mclaren,
                ..Default::default()
            },
            Team {
                team_name: TeamName::AstonMartin,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Alpine,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Williams,
                ..Default::default()
            },
            Team {
                team_name: TeamName::AlphaTauri,
                ..Default::default()
            },
            Team {
                team_name: TeamName::AlphaRomeo,
                ..Default::default()
            },
            Team {
                team_name: TeamName::Haas,
                ..Default::default()
            },
        ];

        // When
        let team_index = driver.find_team(&teams);

        // Then
        assert_eq!(expected_team_index, team_index);
    }
}
