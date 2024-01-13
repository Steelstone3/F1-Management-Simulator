use std::fmt::Display;

use super::race_grid::RaceGrid;

pub const NUMBER_OF_RACES_IN_A_SEASON: usize = 10;

#[derive(Default)]
pub struct Season {
    races: [RaceGrid; NUMBER_OF_RACES_IN_A_SEASON],
}

impl Season {
    pub fn calculate_season_results(&mut self) {
        for race in &mut self.races {
            race.calculate_driver_race_chances();
            let scoring_drivers = race.race_result_order();
            race.assign_points(scoring_drivers);
        }
    }
}

impl Display for Season {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

#[cfg(test)]
mod grid_should {
    use crate::models::season::{Season, NUMBER_OF_RACES_IN_A_SEASON};

    #[test]
    fn new_grid() {
        // Given
        let race_grid = Season::default();

        // Then
        assert_eq!(NUMBER_OF_RACES_IN_A_SEASON, race_grid.races.len())
    }
}
