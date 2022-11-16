use crate::models::{driver::Driver, driver_name::DriverName, race::Race, team_name::TeamName, team::Team};
use inquire::Select;

pub fn display_the_grid(driver_grid: [Driver; 22]) {
    for driver in driver_grid {
        println!("Driver: {} Team: {}", driver.name, driver.team.name);
    }
}

pub fn display_the_drivers_season(season_results: [Driver; 22]) {
    let mut index = 1;

    println!("\n\n\nDriver Season Results:\n");

    for driver in season_results {
        println!(
            "| {} | Driver: {} | Team: {} | Season Points: {} |",
            index,
            driver.name,
            driver.team.name,
            driver.season_points
        );

        index += 1;
    }
}

pub fn display_the_constructors_season(season_results: [Team; 11]) {
    let mut index = 1;

    println!("\n\n\nConstructors Season Results:\n");

    for team in season_results {
        println!(
            "| {} | Team: {} | Season Points: {} |",
            index,
            team.name,
            team.season_points
        );

        index += 1;
    }
}

pub fn display_the_race_result(race: Race) {
    println!("\n\n\nRace: {}\n", race.name);

    for index in 0..race.race_results.len() {
        println!(
            "| {} | Driver: {} | Team: {} | Points: {} |",
            index + 1,
            race.race_results[index].name,
            race.race_results[index].team.name,
            race.race_results[index].race_points
        );
    }
}

pub fn select_driver_name() -> DriverName {
    let options: Vec<DriverName> = vec![DriverName::NyckDeVries, DriverName::NicoHulkenberg];

    Select::new("Select Driver:", options).prompt().unwrap()
}

pub fn select_team_name() -> TeamName {
    let options: Vec<TeamName> = vec![TeamName::Minardi, TeamName::ToroRosso];

    Select::new("Select Team:", options).prompt().unwrap()
}
