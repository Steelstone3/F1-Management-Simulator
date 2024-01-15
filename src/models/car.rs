use rand_derive2::RandGen;

use crate::controller::random_generator::generate_seeded_random;

#[derive(RandGen, Default, Debug, PartialEq, Eq)]
pub struct Car {
    pub aero: u32,
    pub engine: u32,
    pub reliability: u32,
    pub tire_management: u32,
    pub overall: u32,
}

impl Car {
    pub fn new(seeds: [u64; 4]) -> Self {
        let aero = generate_seeded_random(seeds[0]);
        let engine = generate_seeded_random(seeds[1]);
        let reliability = generate_seeded_random(seeds[2]);
        let tire_management = generate_seeded_random(seeds[3]);

        let overall = (aero + engine + reliability + tire_management) / 4;

        Self {
            aero,
            engine,
            reliability,
            tire_management,
            overall,
        }
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1,2,3,4], Car {
        aero: 90,
        engine: 64,
        reliability: 81,
        tire_management: 84,
        overall: 79
    })]
    #[case([100,200,300,400], Car {
        aero: 87,
        engine: 90,
        reliability: 57,
        tire_management: 70,
        overall: 76
    })]
    #[case([1000, 2000, 3000, 4000], Car {
        aero: 60,
        engine: 74,
        reliability: 75,
        tire_management: 85,
        overall: 73
    })]
    fn create_a_random_car(#[case] seeds: [u64; 4], #[case] expected: Car) {
        // Given
        let car = Car::new(seeds);

        // Then
        assert_eq!(expected, car);
    }
}
