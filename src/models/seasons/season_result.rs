use std::fmt::Display;
use crate::models::{races::race_grid::TEAMS_ON_THE_RACE_GRID, teams::team::Team, drivers::driver_name::{self, DriverName}};

// TODO aggrigate result holder
pub struct SeasonResult {
    teams: [Team; TEAMS_ON_THE_RACE_GRID],
}

impl Display for SeasonResult {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

impl SeasonResult {
    pub fn new(teams: [Team; TEAMS_ON_THE_RACE_GRID]) -> Self {
        Self { teams }
    }

    // pub fn aggregate_driver_points(&mut self) {
    //     for team in &self.teams {
            
    //     }
    // }
}
