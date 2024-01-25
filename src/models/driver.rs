use super::{names::{driver_names::DriverName, team_names::TeamName}, points::Points, ratings::driver_rating::DriverRating};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_rating: DriverRating,
    pub points: Points,
}
