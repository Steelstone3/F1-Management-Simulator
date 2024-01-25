#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriverRating {
    pub awareness: u32,
    pub consistency: u32,
    pub experience: u32,
    pub race_craft: u32,
    pub pace: u32,
    pub overall: u32,
    pub overall_race_chance: u32,
}
