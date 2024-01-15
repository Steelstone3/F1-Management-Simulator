use crate::models::{
    races::race_grid::{RaceGrid, TEAMS_ON_THE_RACE_GRID},
    teams::team_seeds::TeamSeeds,
};

use super::season_result::SeasonResult;

pub const NUMBER_OF_RACES_IN_A_SEASON: usize = 10;

#[derive(Clone, Copy)]
pub struct Season {
    pub races: [RaceGrid; NUMBER_OF_RACES_IN_A_SEASON],
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

        let season_result = self.update_driver_season_points(self.races[9]);

        println!("{}", season_result);
    }

    // TODO add a result
    fn update_driver_season_points(&mut self, race_grid: RaceGrid) -> SeasonResult {
        let mut season_result = SeasonResult::new(race_grid.teams);

        for race_number in 0..NUMBER_OF_RACES_IN_A_SEASON {
            for team_result_index in 0..TEAMS_ON_THE_RACE_GRID {
                for team in self.races[race_number].teams {
                    if season_result.results[team_result_index].team_name == team.team_name {
                        season_result.results[team_result_index]
                            .driver_1
                            .driver_season_points
                            .season_points += team.driver_1.driver_race_points.race_points;

                        season_result.results[team_result_index]
                            .driver_2
                            .driver_season_points
                            .season_points += team.driver_2.driver_race_points.race_points;
                    }
                }
            }
        }

        season_result
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
