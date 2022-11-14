use crate::{
    controllers::random_generator::generate_seed,
    models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName},
};
use inquire::Select;
use rand::random;

pub fn print(message: &str) {
    println!("{}", message);
}

pub fn select_driver() -> Driver {
    Driver::new_random([
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
    ])
}

pub fn select_team() -> Team {
    Team {
        name: select_team_name(),
        car: random(),
        points: Default::default(),
    }
}

// TODO use this
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
