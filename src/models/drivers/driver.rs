use crate::{controller::random_generator::generate_seed, models::points::Points};

use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use rand::random;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Driver {
    pub name: DriverName,
    pub statistics: DriverStatistic,
    pub points: Points,
}

impl Driver {
    pub fn new() -> Self {
        let driver_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];

        Self {
            name: random(),
            statistics: DriverStatistic::new(driver_seeds),
            points: Default::default(),
        }
    }
}

impl Display for Driver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}\n{}\n\n\n", self.name, self.statistics)
    }
}

#[cfg(test)]
mod driver_should {}
