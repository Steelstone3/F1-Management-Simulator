use crate::controller::random_generator::{generate_4_seeds, generate_5_seeds};
use std::fmt::Display;

use super::{
    drivers::driver_name::DriverName,
    teams::{team::Team, team_name::TeamName},
};

pub const CARS_ON_THE_RACE_GRID: usize = 10;

pub struct RaceGrid {
    pub teams: [Team; CARS_ON_THE_RACE_GRID],
}

impl RaceGrid {
    pub fn calculate_race(&self) {

        for _driver in 0..CARS_ON_THE_RACE_GRID {
            
        }
    }
}

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
        assert_eq!(CARS_ON_THE_RACE_GRID, race_grid.teams.len())
    }
}
