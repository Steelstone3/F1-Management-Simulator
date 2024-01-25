use super::{names::{driver_names::DriverName, team_names::TeamName}, points::Points};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_rating: DriverRating,
    pub points: Points,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct DriverRating {
    pub awareness: u32,
    pub consistency: u32,
    pub experience: u32,
    pub race_craft: u32,
    pub pace: u32,
    pub overall: u32,
    pub overall_race_chance: u32,
}

