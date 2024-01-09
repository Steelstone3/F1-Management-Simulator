use crate::{controller::random_generator::generate_seed, models::points::Points};

use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Driver {
    pub name: DriverName,
    pub statistics: DriverStatistic,
    pub points: Points,
}

impl Driver {
    pub fn new(driver_name: DriverName) -> Self {
        let driver_seeds = [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ];

        Self {
            name: driver_name,
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

impl Default for Driver {
    fn default() -> Self {
        Self {
            name: Default::default(),
            statistics: Default::default(),
            points: Default::default(),
        }
    }
}

#[cfg(test)]
mod driver_should {}
