use super::{names::{driver_names::DriverName, team_names::TeamName}, points::Points, ratings::driver_rating::DriverRating};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_rating: DriverRating,
    pub points: Points,
}
impl Driver {
    pub fn new(driver_name: DriverName, team_name: TeamName, seeds: [u64; 5]) -> Driver {
        todo!()
    }
}
