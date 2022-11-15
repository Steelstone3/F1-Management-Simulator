use crate::models::{driver_name::DriverName, team_name::TeamName, driver::Driver};
use inquire::Select;

pub fn display_the_grid(driver_grid: [Driver; 20]) {
    for index in 0..20 {
        println!("Driver: {} Team: {}", driver_grid[index].name, driver_grid[index].team.name);
    }
}

pub fn select_driver_name() -> DriverName {
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

pub fn select_team_name() -> TeamName {
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
