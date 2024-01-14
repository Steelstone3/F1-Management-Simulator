use crate::controller::random_generator::{generate_4_seeds, generate_5_seeds};
use std::fmt::Display;

use super::{
    drivers::{driver::Driver, driver_name::DriverName},
    points::RACE_POSIITIONS_THAT_ALLOCATE_POINTS,
    teams::{team::Team, team_name::TeamName},
};

pub const TEAMS_ON_THE_RACE_GRID: usize = 10;

pub struct RaceGrid {
    pub teams: [Team; TEAMS_ON_THE_RACE_GRID],
}

impl RaceGrid {
    pub fn calculate_driver_race_chances(&mut self) {
        for team in 0..TEAMS_ON_THE_RACE_GRID {
            self.teams[team].calculate_drivers_overall_race_chance()
        }
    }

    pub fn race_result_order(&self) -> [Driver; RACE_POSIITIONS_THAT_ALLOCATE_POINTS] {
        // TODO
        // Order drivers based on race chances all 20
        // Display driver result order all 20
        // Return race positions that allocate points all 10
        let mut drivers = vec![];

        for team in &self.teams {
            drivers.push(team.driver_1);
            drivers.push(team.driver_2);
        }

        drivers.sort_by(|a, b| b.overall_race_chance.cmp(&a.overall_race_chance));

        for driver in &drivers {
            println!("{}", driver);
        }

        let top_ten_drivers = drivers
            .into_iter()
            .take(10)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        top_ten_drivers
    }

    pub fn assign_points(&mut self, drivers: [Driver; RACE_POSIITIONS_THAT_ALLOCATE_POINTS]) {
        // TODO
        // Take each of the drivers
        // Match the driver by name
        // Add the matched drivers points based on race position for the correct race in the season
    }
}

// TODO update display to show the race result
impl Display for RaceGrid {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

impl Default for RaceGrid {
    fn default() -> Self {
        Self {
            teams: [
                Team::new(
                    TeamName::RedBull,
                    DriverName::MaxVerstappen,
                    DriverName::SergioPerez,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Ferrari,
                    DriverName::CharlesLeclerc,
                    DriverName::CarlosSainz,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Mercedes,
                    DriverName::LewisHamilton,
                    DriverName::GeorgeRussell,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Mclaren,
                    DriverName::LandoNorris,
                    DriverName::DanielRicciardo,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::AstonMartin,
                    DriverName::SebastianVettel,
                    DriverName::LanceStroll,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Alpine,
                    DriverName::EstebanOcon,
                    DriverName::FernandoAlonso,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Williams,
                    DriverName::AlexanderAlbon,
                    DriverName::NicholasLatifi,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::AlphaTauri,
                    DriverName::PierreGasly,
                    DriverName::YukiTsunoda,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::AlphaRomeo,
                    DriverName::ValtteriBottas,
                    DriverName::GuanyuZhou,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
                Team::new(
                    TeamName::Haas,
                    DriverName::KevinMagnussen,
                    DriverName::MickSchumacher,
                    generate_5_seeds(),
                    generate_4_seeds(),
                    generate_5_seeds(),
                    generate_5_seeds(),
                ),
            ],
        }
    }
}

#[cfg(test)]
mod grid_should {
    use super::*;

    #[test]
    fn new_grid() {
        // Given
        let race_grid = RaceGrid::default();

        // Then
        assert_eq!(TEAMS_ON_THE_RACE_GRID, race_grid.teams.len())
    }

    #[test]
    // #[ignore = "not passing"]
    fn calculate_driver_finishing_positions() {
        // Given
        let race_grid = RaceGrid {
            teams: [
                Team {
                    driver_1: Driver {
                        overall_race_chance: 99,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 95,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 80,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 70,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 90,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 85,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 75,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 65,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 60,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 55,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Team {
                    driver_1: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    driver_2: Driver {
                        overall_race_chance: 50,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
        };
        let expected_race_result = [
            Driver {
                overall_race_chance: 99,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 95,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 90,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 85,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 80,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 75,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 70,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 65,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 60,
                ..Default::default()
            },
            Driver {
                overall_race_chance: 55,
                ..Default::default()
            },
        ];

        // When
        let race_result = race_grid.race_result_order();

        // Then
        assert_eq!(expected_race_result, race_result);
    }
}
