use super::{car::Car, driver::Driver, team_name::TeamName};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, PartialEq)]
pub struct Team {
    team_name: TeamName,
    car: Car,
    driver_1: Driver,
    driver_2: Driver,
    points: u16,
}

impl Team {
    fn new(team_name: TeamName, car: Car, driver_1: Driver, driver_2: Driver) -> Self {
        Self {
            team_name,
            car,
            driver_1,
            driver_2,
            points: Default::default(),
        }
    }

    fn calculate_race_chances(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let race_factor_1 = rng.gen_range(0.8..1.2);
        let race_factor_2 = rng.gen_range(0.8..1.2);

        self.driver_1.race_chances =
            (self.driver_1.overall + self.car.overall) as f32 * race_factor_1;

        self.driver_2.race_chances =
            (self.driver_2.overall + self.car.overall) as f32 * race_factor_2;
    }

    fn calculate_points(&mut self) {
        self.points += self.driver_1.points + self.driver_2.points;
    }
}

#[cfg(test)]
mod team_should {
    use super::*;
    use crate::models::driver_name::DriverName;

    #[test]
    fn create_a_team() {
        let expected_team = team_test_fixture();

        let team = Team::new(
            TeamName::Haas,
            Car {
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
            Driver {
                driver_name: DriverName::CharlesLeclerc,
                expierence: 4,
                race_craft: 75,
                awareness: 60,
                pace: 99,
                overall: 99,
                race_chances: 25.0,
                points: 10,
            },
            Driver {
                driver_name: DriverName::CarlosSainz,
                expierence: 3,
                race_craft: 67,
                awareness: 80,
                pace: 85,
                overall: 99,
                race_chances: 300.0,
                points: 20,
            },
        );

        assert_eq!(expected_team, team);
    }

    #[test]
    fn work_out_race_chances() {
        let driver_1_race_chances = 193.71185;
        let driver_2_race_chances = 176.7328;
        let mut team = team_test_fixture();

        team.calculate_race_chances(2022);

        assert_eq!(driver_1_race_chances, team.driver_1.race_chances);
        assert_eq!(driver_2_race_chances, team.driver_2.race_chances);
        assert_ne!(team.driver_1.race_chances, team.driver_2.race_chances);
    }

    #[test]
    fn calculate_team_points() {
        let mut team = team_test_fixture();

        team.calculate_points();
        team.calculate_points();

        assert_eq!(60, team.points)
    }

    fn team_test_fixture() -> Team {
        Team {
            team_name: TeamName::Haas,
            car: Car {
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
            driver_1: Driver {
                driver_name: DriverName::CharlesLeclerc,
                expierence: 4,
                race_craft: 75,
                awareness: 60,
                pace: 99,
                overall: 99,
                race_chances: 25.0,
                points: 10,
            },
            driver_2: Driver {
                driver_name: DriverName::CarlosSainz,
                expierence: 3,
                race_craft: 67,
                awareness: 80,
                pace: 85,
                overall: 99,
                race_chances: 300.0,
                points: 20,
            },
            points: Default::default(),
        }
    }
}
