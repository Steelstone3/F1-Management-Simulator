use std::fmt::Display;

use super::race_grid::RaceGrid;

pub const NUMBER_OF_RACES: usize = 10;

pub struct Season {
    races: [RaceGrid; NUMBER_OF_RACES],
}

impl Season {
    pub fn calculate_season() {
        
    }
}

impl Display for Season {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

impl Default for Season {
    fn default() -> Self {
        Self {
            races: Default::default(),
        }
    }
}

#[cfg(test)]
mod grid_should {
    use crate::models::season::{Season, NUMBER_OF_RACES};

    #[test]
    fn new_grid() {
        // Given
        let race_grid = Season::default();

        // Then
        assert_eq!(NUMBER_OF_RACES, race_grid.races.len())
    }
}
