use super::{
    driver::Driver,
    names::team_names::TeamName,
    ratings::{car_rating::CarRating, team_rating::TeamRating},
};

#[derive(Clone, Copy)]
pub struct Team {
    pub team_name: TeamName,
    pub team_rating: TeamRating,
    pub car: CarRating,
    pub driver_1: Driver,
    pub driver_2: Driver,
}
