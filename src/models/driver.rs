use rand::random;
use rand_derive2::RandGen;

use crate::controllers::random_generator::assign_random_range;

use super::driver_name::DriverName;

#[derive(RandGen, Debug, PartialEq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub expierence: u8,
    pub race_craft: u8,
    pub awareness: u8,
    pub pace: u8,
    pub overall: u32,
    pub points: u16,
}

impl Driver {
    pub fn new(
        driver_name: DriverName,
        expierence: u8,
        race_craft: u8,
        awareness: u8,
        pace: u8,
    ) -> Self {
        let mut driver = Self {
            driver_name,
            expierence,
            race_craft,
            awareness,
            pace,
            overall: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        driver
    }

    fn calculate_overall(&mut self) {
        // calculate the average of the stats
        self.overall = ((self.expierence as u32 * 6)
            + self.race_craft as u32
            + self.awareness as u32
            + self.pace as u32)
            / 4;
    }

    fn add_points(&mut self, points: u16) {
        self.points += points;
    }
}

impl Default for Driver {
    fn default() -> Self {
        const MAX_EXPIERENCE: u8 = 25;
        const MAX_RANGE: u8 = 99;

        let mut driver = Self {
            driver_name: random(),
            expierence: assign_random_range(MAX_EXPIERENCE),
            race_craft: assign_random_range(MAX_RANGE),
            awareness: assign_random_range(MAX_RANGE),
            pace: assign_random_range(MAX_RANGE),
            overall: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        driver
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(DriverName::CharlesLeclerc, 4, 75, 60, 99, 64, 0)]
    #[case(DriverName::LewisHamilton, 15, 99, 95, 97, 95, 0)]
    #[case(DriverName::MaxVerstappen, 7, 80, 75, 99, 74, 0)]
    fn create_a_driver(
        #[case] driver_name: DriverName,
        #[case] expierence: u8,
        #[case] race_craft: u8,
        #[case] awareness: u8,
        #[case] pace: u8,
        #[case] overall: u32,
        #[case] points: u16,
    ) {
        let expected_driver = Driver {
            driver_name,
            expierence,
            race_craft,
            awareness,
            pace,
            overall,
            points
        };

        let driver = Driver::new(driver_name, expierence, race_craft, awareness, pace);

        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn create_a_random_driver() {
        let random_driver = Driver::default();

        assert!(!random_driver.expierence > 25);
        assert!(!random_driver.race_craft > 99);
        assert!(!random_driver.awareness > 99);
        assert!(!random_driver.pace > 99);
        assert!(!random_driver.overall > 99);
        assert_eq!(0, random_driver.points );
    }

    #[test]
    fn add_driver_points() {
        let points = 1000;
        let mut driver = Driver::default();

        driver.add_points(points);

        assert_eq!(points, driver.points)
    }
}
