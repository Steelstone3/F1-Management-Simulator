use super::{car::Car, team_name::TeamName};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_derive2::RandGen;

#[derive(RandGen, Clone, Copy, Debug, PartialEq)]
pub struct Team {
    pub team_name: TeamName,
    pub car: Car,
    pub points: u16,
}

impl Team {
    fn new(team_name: TeamName, car: Car) -> Self {
        Self {
            team_name,
            car,
            points: Default::default(),
        }
    }

    // pub fn calculate_race_chances(&mut self, seed: u64) {
    //     let mut rng = StdRng::seed_from_u64(seed);
    //     let race_factor_1 = rng.gen_range(0.8..1.2);
    //     let race_factor_2 = rng.gen_range(0.8..1.2);

    //     self.driver_1.race_chances =
    //         (self.driver_1.overall + self.car.overall) as f32 * race_factor_1;

    //     self.driver_2.race_chances =
    //         (self.driver_2.overall + self.car.overall) as f32 * race_factor_2;
    // }
}

#[cfg(test)]
mod team_should {
    use super::*;

    #[test]
    fn create_a_team() {
        let expected_team = Team {
            team_name: TeamName::Haas,
            car: Car {
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
            points: Default::default(),
        };

        let team = Team::new(
            TeamName::Haas,
            Car {
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
        );

        assert_eq!(expected_team, team);
    }
}
