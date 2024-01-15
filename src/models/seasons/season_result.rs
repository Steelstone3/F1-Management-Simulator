use crate::models::{races::race_grid::TEAMS_ON_THE_RACE_GRID, teams::team::Team};
use std::fmt::Display;

pub struct SeasonResult {
    pub results: [Team; TEAMS_ON_THE_RACE_GRID],
}

impl Display for SeasonResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Season Results");

        let mut team_string = String::new();

        for team in self.results {
            team_string += &team.to_string();
        }

        write!(formatter, "\n\n{}", team_string,)
    }
}

impl SeasonResult {
    pub fn new(teams: [Team; TEAMS_ON_THE_RACE_GRID]) -> Self {
        Self { results: teams }
    }

    pub fn add_season_points(&mut self, team_result_index: usize, team: Team) {
        self.results[team_result_index]
            .driver_1
            .driver_season_points
            .season_points += team.driver_1.driver_race_points.race_points;

        self.results[team_result_index]
            .driver_2
            .driver_season_points
            .season_points += team.driver_2.driver_race_points.race_points;
    }
}
