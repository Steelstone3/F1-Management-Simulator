use super::race_grid::RaceGrid;

pub const NUMBER_OF_RACES: usize = 10;

pub struct Season {
    races: [RaceGrid; NUMBER_OF_RACES]
}