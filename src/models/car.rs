use crate::controllers::random_generator::get_seeded_random_max_range;
use rand_derive2::RandGen;

#[derive(RandGen, Clone, Copy, Debug, PartialEq)]
pub struct Car {
    pub aero: u8,
    pub engine: u8,
    pub reliability: u8,
    pub tire_management: u8,
    pub overall: u32,
}

impl Car {
    pub fn new_random(seeds: [u64; 4]) -> Self {
        const MAX_RANGE: u8 = 99;

        let mut car = Self {
            aero: get_seeded_random_max_range(seeds[0], MAX_RANGE),
            engine: get_seeded_random_max_range(seeds[1], MAX_RANGE),
            reliability: get_seeded_random_max_range(seeds[2], MAX_RANGE),
            tire_management: get_seeded_random_max_range(seeds[3], MAX_RANGE),
            overall: Default::default(),
        };

        car.calculate_overall();

        car
    }

    fn calculate_overall(&mut self) {
        // calculate the average of the stats
        self.overall = (self.aero as u32
            + self.engine as u32
            + self.reliability as u32
            + self.tire_management as u32)
            / 4;
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_a_car() {
        let expected_car = Car {
            aero: 81,
            engine: 8,
            reliability: 64,
            tire_management: 69,
            overall: 55,
        };

        let car = Car::new_random([1, 2, 3, 4]);

        assert_eq!(expected_car, car);
    }

    #[rstest]
    #[case(1, 1, 1, 1, 1)]
    #[case(10, 20, 30, 40, 25)]
    #[case(50, 90, 20, 70, 57)]
    fn calculate_overall_car_stat(
        #[case] aero: u8,
        #[case] engine: u8,
        #[case] reliability: u8,
        #[case] tire_management: u8,
        #[case] overall: u32,
    ) {
        let mut car = Car {
            aero,
            engine,
            reliability,
            tire_management,
            overall: Default::default(),
        };

        car.calculate_overall();

        assert_eq!(overall, car.overall);
    }
}
