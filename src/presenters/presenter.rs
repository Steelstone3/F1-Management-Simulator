use crate::models::{driver_name::DriverName, team_name::TeamName, driver::Driver};
use inquire::Select;

pub fn display_the_grid(driver_grid: [Driver; 22]) {
    for index in 0..22 {
        println!("Driver: {} Team: {}", driver_grid[index].name, driver_grid[index].team.name);
    }
}

pub fn select_driver_name() -> DriverName {
    let options: Vec<DriverName> = vec![
        DriverName::NyckDeVries,
        DriverName::NicoHulkenberg,
    ];

    Select::new("Select Driver:", options).prompt().unwrap()
}

pub fn select_team_name() -> TeamName {
    let options: Vec<TeamName> = vec![
        TeamName::Minardi,
        TeamName::ToroRosso
    ];

    Select::new("Select Team:", options).prompt().unwrap()
}
