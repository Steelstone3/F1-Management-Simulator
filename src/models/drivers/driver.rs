use super::{driver_name::DriverName, driver_statistics::DriverStatistic};
use crate::models::points::Points;
use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub driver_statistics: DriverStatistic,
    pub driver_points: Points,
}

impl Driver {
    pub fn new(driver_name: DriverName, driver_statistics_seeds: [u64; 5]) -> Self {
        Self {
            driver_name,
            driver_statistics: DriverStatistic::new(driver_statistics_seeds),
            driver_points: Default::default(),
        }
    }
}

impl Display for Driver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}\n{}\n\n\n",
            self.driver_name, self.driver_statistics
        )
    }
}

#[cfg(test)]
mod driver_should {
    use crate::models::{
        drivers::{driver::Driver, driver_name::DriverName, driver_statistics::DriverStatistic},
        points::Points,
    };

    #[test]
    fn new_driver() {
        // Given
        let expected_driver = Driver {
            driver_name: DriverName::LewisHamilton,
            driver_statistics: DriverStatistic {
                awareness: 90,
                consistency: 64,
                experience: 81,
                race_craft: 84,
                pace: 66,
                overall: 77,
            },
            driver_points: Points::default(),
        };

        // When
        let driver = Driver::new(DriverName::LewisHamilton, [1, 2, 3, 4, 5]);

        // Then
        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn display_driver() {
        // Given
        let expected_driver_display = "Lewis Hamilton\nAwareness: 90\nConsistency: 64\nExpierence: 81\nPace: 66\nRace Craft: 84\nOverall: 77\n\n\n".to_string();
        let driver = Driver {
            driver_name: DriverName::LewisHamilton,
            driver_statistics: DriverStatistic {
                awareness: 90,
                consistency: 64,
                experience: 81,
                race_craft: 84,
                pace: 66,
                overall: 77,
            },
            driver_points: Points::default(),
        };

        // When
        let driver_display = driver.to_string();

        // Then
        assert_eq!(expected_driver_display, driver_display);
    }
}
