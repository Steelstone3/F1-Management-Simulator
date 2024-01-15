use crate::models::{races::race_grid::TEAMS_ON_THE_RACE_GRID, teams::team::Team};

// TODO aggrigate result holder
pub struct SeasonResult {
    teams: [Team; TEAMS_ON_THE_RACE_GRID],
}

impl SeasonResult {
    pub fn new(teams: [Team; TEAMS_ON_THE_RACE_GRID]) -> Self {
        Self { teams }
    }
}
