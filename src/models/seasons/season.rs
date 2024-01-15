use std::fmt::Display;

use crate::models::{
    races::race_grid::RaceGrid,
    teams::{team::Team, team_name::TeamName, team_seeds::TeamSeeds},
};

use super::season_result::{self, SeasonResult};

pub const NUMBER_OF_RACES_IN_A_SEASON: usize = 10;

pub struct Season {
    races: [RaceGrid; NUMBER_OF_RACES_IN_A_SEASON],
}

impl Default for Season {
    fn default() -> Self {
        let team_seeds = TeamSeeds::default().team_seeds;
        Self {
            races: [
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
            ],
        }
    }
}

impl Season {
    pub fn calculate_race_results(&mut self) {
        let mut race_number = 0;

        for race in &mut self.races {
            race_number += 1;

            race.display_race_information(race_number);

            race.calculate_driver_race_chances();
            let scoring_drivers = race.race_result_order();
            race.assign_points(scoring_drivers, race_number);
            let race_result = race.get_drivers_on_the_race_grid();

            RaceGrid::display_race_results(race_result);
        }
    }

    fn get_team_from_each_race(&self, team_name: TeamName) -> [Team; NUMBER_OF_RACES_IN_A_SEASON] {
        let mut team_at_races = vec![];

        for race in &self.races {
            for team in race.teams {
                if team.team_name == team_name {
                    team_at_races.push(team)
                }
            }
        }

        team_at_races.try_into().unwrap()
    }
}

impl Display for Season {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Season Results\n{}", self.races[9].teams[0])
    }
}

#[cfg(test)]
mod grid_should {
    use crate::models::seasons::season::{Season, NUMBER_OF_RACES_IN_A_SEASON};

    #[test]
    fn new_grid() {
        // Given
        let race_grid = Season::default();

        // Then
        assert_eq!(NUMBER_OF_RACES_IN_A_SEASON, race_grid.races.len())
    }
}
