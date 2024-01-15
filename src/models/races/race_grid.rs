use std::fmt::Display;
use rand::random;
use super::race_information::RaceInformation;
use crate::{
    controller::random_generator::{generate_4_seeds, generate_5_seeds},
    models::{
        drivers::{driver::Driver, driver_name::DriverName},
        points::{Points, RACE_POSIITIONS_THAT_ALLOCATE_POINTS},
        teams::{team::Team, team_name::TeamName},
    },
};

pub const DRIVERS_ON_THE_RACE_GRID: usize = 20;
pub const TEAMS_ON_THE_RACE_GRID: usize = 10;

pub struct RaceGrid {
    pub teams: [Team; TEAMS_ON_THE_RACE_GRID],
    pub race_information: RaceInformation,
}

impl RaceGrid {
    pub fn display_race_information(&mut self, race_number: u32) {
        self.race_information.race_number = race_number;

        // TODO use inquire and make a table or something
        println!(
            "\n\nRace {} | {}\n",
            self.race_information.race_number, self.race_information.race_track_name
        );
    }

    pub fn calculate_driver_race_chances(&mut self) {
        for team in 0..TEAMS_ON_THE_RACE_GRID {
            self.teams[team].calculate_drivers_overall_race_chance()
        }
    }

    pub fn race_result_order(&self) -> [Driver; RACE_POSIITIONS_THAT_ALLOCATE_POINTS] {
        let mut drivers = self.get_drivers_on_the_race_grid();

        drivers.sort_by(|a, b| b.overall_race_chance.cmp(&a.overall_race_chance));

        drivers
            .into_iter()
            .take(10)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    // TODO test
    pub fn assign_points(
        &mut self,
        mut drivers: [Driver; RACE_POSIITIONS_THAT_ALLOCATE_POINTS],
        race_number: u32,
    ) {
        let race_number = (race_number - 1) as usize;

        for driver_position in 0..RACE_POSIITIONS_THAT_ALLOCATE_POINTS {
            drivers[driver_position].driver_points.race_points[race_number] =
                Points::calculate_points_for_finish_position(driver_position);
        }

        for driver in drivers {
            let team_index = driver.find_team(&self.teams);
            self.teams[team_index].add_points(driver, race_number);
        }
    }

    pub fn display_race_results(drivers: [Driver; DRIVERS_ON_THE_RACE_GRID]) {
        for driver in &drivers {
            println!("{}", driver);
        }
    }

    // TODO test
    pub fn get_drivers_on_the_race_grid(&self) -> [Driver; DRIVERS_ON_THE_RACE_GRID] {
        let mut drivers = vec![];

        for team in &self.teams {
            drivers.push(team.driver_1);
            drivers.push(team.driver_2);
        }

        drivers.try_into().unwrap()
    }
}

impl Display for RaceGrid {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut team_results = String::new();

        for team in &self.teams {
            team_results += &team.to_string();
        }

        write!(formatting, "{}", team_results)
    }
}

// TODO remove default from race grid and go to seeded new that is tested
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
            race_information: RaceInformation {
                race_number: Default::default(),
                race_track_name: random(),
            },
        }
    }
}

#[cfg(test)]
mod grid_should {
    use crate::models::races::race_track_name::RaceTrackName;

    use super::*;

    #[test]
    fn new_grid() {
        // Given
        let race_grid = RaceGrid::default();

        // Then
        assert_eq!(TEAMS_ON_THE_RACE_GRID, race_grid.teams.len())
    }

    #[test]
    fn display_the_race_information() {
        // Given
        let race_number = 1;
        let mut race_grid = RaceGrid {
            race_information: RaceInformation {
                race_number,
                race_track_name: RaceTrackName::Silverstone,
            },
            ..Default::default()
        };

        // When
        race_grid.display_race_information(race_number);

        // Then
        assert_eq!(race_number, race_grid.race_information.race_number)
    }

    #[test]
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
            ..Default::default()
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
