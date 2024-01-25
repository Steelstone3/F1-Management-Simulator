use super::team::Team;

pub const NUMBER_OF_RACES: usize = 10;
pub const NUMBER_OF_TEAMS: usize = 10;

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct Season {
    teams: [Team; NUMBER_OF_TEAMS]
}