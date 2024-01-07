use rand::random;

use super::{driver_name::DriverName, driver_statistics::DriverStatistic};

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

#[cfg(test)]
mod driver_should {
    use super::*;

    #[test]
    fn new_driver() {
        // When
        let driver = Driver::new([1, 2, 3, 4, 5]);

        // Then
        assert_ne!("".to_string(), driver.name.to_string());
        assert_ne!(100, driver.statistics.awareness);
        assert_ne!(49, driver.statistics.awareness);
        assert_ne!(100, driver.statistics.consistency);
        assert_ne!(49, driver.statistics.consistency);
        assert_ne!(100, driver.statistics.experience);
        assert_ne!(49, driver.statistics.experience);
        assert_ne!(100, driver.statistics.race_craft);
        assert_ne!(49, driver.statistics.race_craft);
        assert_ne!(100, driver.statistics.pace);
        assert_ne!(49, driver.statistics.pace);
        assert_ne!(100, driver.statistics.overall);
        assert_ne!(49, driver.statistics.overall);
    }
}
