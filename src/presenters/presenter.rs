use crate::{
    controllers::random_generator::generate_seed,
    models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName},
};
use inquire::Select;

pub fn print(message: &str) {
    println!("{}", message);
}

pub fn select_driver_and_team() -> Driver {
    Driver::new_player(
        select_driver_name(),
        select_team(),
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}

fn select_team() -> Team {
    Team::new(
        select_team_name(),
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}

fn select_driver_name() -> DriverName {
    let options: Vec<DriverName> = vec![
        DriverName::LewisHamilton,
        DriverName::GeorgeRussell,
        DriverName::CharlesLeclerc,
        DriverName::CarlosSainz,
        DriverName::MaxVerstappen,
        DriverName::SergioPerez,
    ];

    Select::new("Select Driver:", options).prompt().unwrap()
}

fn select_team_name() -> TeamName {
    let options: Vec<TeamName> = vec![
        TeamName::Mercedes,
        TeamName::Mclaren,
        TeamName::Haas,
        TeamName::Ferrari,
        TeamName::RedBull,
        TeamName::AstonMartin,
        TeamName::Alpine,
        TeamName::AlphaRomeo,
        TeamName::AlphaTauri,
        TeamName::Williams,
    ];

    Select::new("Select Team:", options).prompt().unwrap()
}
