use crate::models::{teams::team::Team, races::race_grid::TEAMS_ON_THE_RACE_GRID};

// TODO aggrigate result holder
pub struct SeasonResult {
    teams: [Team; TEAMS_ON_THE_RACE_GRID]
}

impl SeasonResult {
    fn new(teams: [Team; TEAMS_ON_THE_RACE_GRID]) -> Self {
        Self {
            teams,
        }
    }
}