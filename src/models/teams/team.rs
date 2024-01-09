use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::{
    controller::random_generator::generate_seed,
    models::{car::Car, drivers::driver::Driver},
};
use rand::random;

#[derive(Debug, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub statistics: TeamStatistic,
    pub car: Car,
    pub drivers: [Driver; 2],
}

impl Team {
    pub fn new() -> Self {
        let team_statistics_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];
        let car_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];

        Self {
            name: random(),
            statistics: TeamStatistic::new(team_statistics_seeds),
            car: Car::new(car_seeds),
            drivers: [Driver::new(), Driver::new()],
        }
    }
}

#[cfg(test)]
mod team_should {}
