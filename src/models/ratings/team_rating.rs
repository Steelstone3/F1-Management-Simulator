#[derive(Clone, Copy)]
pub struct TeamRating {
    pub car_development: u32,
    pub car_repairs: u32,
    pub car_setup: u32,
    pub pitstops: u32,
    pub team_management: u32,
    pub overall: u32,
}
