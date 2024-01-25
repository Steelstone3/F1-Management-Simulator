use super::season::NUMBER_OF_RACES;

#[derive(Copy, Clone)]
pub struct Points {
    pub race_points: [u32; NUMBER_OF_RACES],
    pub season_points: u32,
}
