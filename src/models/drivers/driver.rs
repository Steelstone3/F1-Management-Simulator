use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use rand::random;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Driver {
    pub name: DriverName,
    pub statistics: DriverStatistic,
}

impl Driver {
    pub fn new(seeds: [u64; 5]) -> Self {
        Self {
            name: random(),
            statistics: DriverStatistic::new(seeds),
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
