use crate::controllers::random_generator::assign_random_range;

#[derive(Debug, PartialEq)]
pub struct Car {
    pub aero: u8,
    pub engine: u8,
    pub reliability: u8,
    pub tire_management: u8,
    pub overall: u32,
}

impl Car {
    fn calculate_overall(&mut self) {
        // calculate the average of the stats
        self.overall = (self.aero as u32
            + self.engine as u32
            + self.reliability as u32
            + self.tire_management as u32)
            / 4;
    }
}

impl Default for Car {
    fn default() -> Self {
        const MAX_RANGE: u8 = 99;

        let mut car = Self {
            aero: assign_random_range(MAX_RANGE),
            engine: assign_random_range(MAX_RANGE),
            reliability: assign_random_range(MAX_RANGE),
            tire_management: assign_random_range(MAX_RANGE),
            overall: Default::default(),
        };

        car.calculate_overall();

        car
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;

    #[test]
    fn create_a_car() {
        let random_driver = Car::default();

        assert!(!random_driver.aero > 99);
        assert!(!random_driver.engine > 99);
        assert!(!random_driver.reliability > 99);
        assert!(!random_driver.tire_management > 99);
        assert!(!random_driver.overall > 99);
    }
}
