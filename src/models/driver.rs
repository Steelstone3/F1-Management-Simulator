use super::driver_name::DriverName;
use crate::controllers::random_generator::get_seeded_random_max_range;
use rand::random;
use rand_derive2::RandGen;

#[derive(RandGen, Debug, PartialEq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub expierence: u8,
    pub race_craft: u8,
    pub awareness: u8,
    pub pace: u8,
    pub overall: u32,
    pub race_chances: f32,
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
            race_chances: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        driver
    }

    fn new_random(seeds: [u64; 4]) -> Self {
        const MAX_EXPIERENCE: u8 = 25;
        const MAX_RANGE: u8 = 99;

        let mut driver = Self {
            driver_name: random(),
            expierence: get_seeded_random_max_range(seeds[0], MAX_EXPIERENCE),
            race_craft: get_seeded_random_max_range(seeds[1], MAX_RANGE),
            awareness: get_seeded_random_max_range(seeds[2], MAX_RANGE),
            pace: get_seeded_random_max_range(seeds[3], MAX_RANGE),
            overall: Default::default(),
            race_chances: Default::default(),
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

#[cfg(test)]
mod driver_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(DriverName::CharlesLeclerc, 4, 75, 60, 99, 64, 0.0, 0)]
    #[case(DriverName::LewisHamilton, 15, 99, 95, 97, 95, 0.0, 0)]
    #[case(DriverName::MaxVerstappen, 7, 80, 75, 99, 74, 0.0, 0)]
    fn create_a_driver(
        #[case] driver_name: DriverName,
        #[case] expierence: u8,
        #[case] race_craft: u8,
        #[case] awareness: u8,
        #[case] pace: u8,
        #[case] overall: u32,
        #[case] race_chances: f32,
        #[case] points: u16,
    ) {
        let expected_driver = Driver {
            driver_name,
            expierence,
            race_craft,
            awareness,
            pace,
            overall,
            race_chances,
            points,
        };

        let driver = Driver::new(driver_name, expierence, race_craft, awareness, pace);

        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn create_a_random_driver() {
        let expected_driver = Driver {
            driver_name: DriverName::LewisHamilton,
            expierence: 20,
            race_craft: 8,
            awareness: 64,
            pace: 69,
            overall: 65,
            race_chances: Default::default(),
            points: Default::default(),
        };

        let driver = Driver::new_random([1, 2, 3, 4]);

        assert_eq!(expected_driver.expierence, driver.expierence);
        assert_eq!(expected_driver.race_craft, driver.race_craft);
        assert_eq!(expected_driver.awareness, driver.awareness);
        assert_eq!(expected_driver.pace, driver.pace);
        assert_eq!(expected_driver.overall, driver.overall);
        assert_eq!(expected_driver.race_chances, driver.race_chances);
        assert_eq!(expected_driver.points, driver.points);
    }

    #[test]
    fn add_driver_points() {
        let expected_points = 2000;
        let points = 1000;
        let mut driver = Driver {
            driver_name: DriverName::LewisHamilton,
            expierence: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: Default::default(),
            race_chances: Default::default(),
            points: Default::default(),
        };

        driver.add_points(points);
        driver.add_points(points);

        assert_eq!(expected_points, driver.points)
    }
}
