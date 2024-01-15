use crate::{
    controller::random_generator::{generate_4_seeds, generate_5_seeds},
    models::races::race_grid::TEAMS_ON_THE_RACE_GRID,
};

#[derive(Clone, Copy)]
#[derive(Default)]
pub struct TeamSeeds {
    pub team_seeds: [TeamSeed; TEAMS_ON_THE_RACE_GRID],
}



#[derive(Clone, Copy)]
pub struct TeamSeed {
    pub team_statistics_seeds: [u64; 5],
    pub car_seeds: [u64; 4],
    pub driver_1_seeds: [u64; 5],
    pub driver_2_seeds: [u64; 5],
}

impl Default for TeamSeed {
    fn default() -> Self {
        Self {
            team_statistics_seeds: generate_5_seeds(),
            car_seeds: generate_4_seeds(),
            driver_1_seeds: generate_5_seeds(),
            driver_2_seeds: generate_5_seeds(),
        }
    }
}
