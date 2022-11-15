use crate::{
    models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName}, presenters::presenter::{select_driver_name, select_team_name},
};

use super::random_generator::generate_seed;

pub fn create_grid() -> [Driver; 20] {
    [
        create_driver(select_driver_name(), select_team_name()),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::GeorgeRussell, TeamName::Mercedes),
        create_driver(DriverName::CharlesLeclerc, TeamName::Ferrari),
        create_driver(DriverName::CarlosSainz, TeamName::Ferrari),
        create_driver(DriverName::MaxVerstappen, TeamName::RedBull),
        create_driver(DriverName::SergioPerez, TeamName::RedBull),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
    ]
}

fn create_driver(driver_name: DriverName, team_name: TeamName) -> Driver{
    Driver::new_player(
        driver_name,
        Team::new(
            team_name,
            [
                generate_seed(),
                generate_seed(),
                generate_seed(),
                generate_seed(),
            ],
        ),
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}