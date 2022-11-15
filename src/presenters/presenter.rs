use crate::models::{driver_name::DriverName, team_name::TeamName, driver::Driver, race::Race};
use inquire::Select;

pub fn display_the_grid(driver_grid: [Driver; 22]) {
    for driver in driver_grid {
        println!("Driver: {} Team: {}", driver.name, driver.team.name);
    }
}

pub fn display_the_race_result(race: Race) {
    println!("\n\n\nRace: {}\n", race.name);

    for index in 0..22 {
        println!("| {} | Driver: {} | Team: {} | Points: {} |", index + 1, race.race_results[index].name, race.race_results[index].team.name, race.race_results[index].points);
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
        TeamName::ToroRosso,
    ];

    Select::new("Select Team:", options).prompt().unwrap()
}
