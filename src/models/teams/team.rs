use super::{team_name::TeamName, team_statistics::TeamStatistic};
use crate::models::{car::Car, drivers::driver::Driver};
use rand::random;

#[derive(Debug, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub statistics: TeamStatistic,
    pub car: Car,
    pub drivers: [Driver; 2],
}

impl Team {
    pub fn new(
        team_seeds: [u64; 5],
        car_seeds: [u64; 4],
        driver_1_seeds: [u64; 5],
        driver_2_seeds: [u64; 5],
    ) -> Self {
        Self {
            name: random(),
            statistics: TeamStatistic::new(team_seeds),
            car: Car::new(car_seeds),
            drivers: [Driver::new(driver_1_seeds), Driver::new(driver_2_seeds)],
        }
    }
}

#[cfg(test)]
mod team_should {}
