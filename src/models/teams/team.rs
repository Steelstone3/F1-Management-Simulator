use crate::models::{car::Car, drivers::driver::Driver};
use super::team_name::TeamName;

#[derive(Debug, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub car: Car,
    pub drivers: [Driver; 2],
}

#[cfg(test)]
mod team_should {}
