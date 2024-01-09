use crate::models::{car::Car, drivers::driver::Driver};
use super::{team_name::TeamName, team_statistics::TeamStatistic};

#[derive(Debug, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub statistics: TeamStatistic,
    pub car: Car,
    pub drivers: [Driver; 2],
}

#[cfg(test)]
mod team_should {}
