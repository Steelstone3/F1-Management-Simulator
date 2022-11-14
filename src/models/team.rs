use super::{car::Car, driver::Driver, team_name::TeamName};

const BUDGET: u64 = 150000000;

#[derive(Debug, PartialEq)]
pub struct Team {
    team_name: TeamName,
    car: Car,
    driver_1: Driver,
    driver_2: Driver,
    reserve_driver: Driver,
    budget: u64,
    points: u16,
}

impl Team {
    fn new(
        team_name: TeamName,
        car: Car,
        driver_1: Driver,
        driver_2: Driver,
        reserve_driver: Driver,
    ) -> Self {
        Self {
            team_name,
            car,
            driver_1,
            driver_2,
            reserve_driver,
            budget: BUDGET,
            points: Default::default()
        }
    }

    fn calculate_points(&mut self) {
        self.points = self.driver_1.points + self.driver_2.points + self.reserve_driver.points;
    }
}

#[cfg(test)]
mod team_name_should {
    use super::*;
    use crate::models::driver_name::DriverName;

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
            driver_1: Driver {
                driver_name: DriverName::CharlesLeclerc,
                expierence: 4,
                race_craft: 75,
                awareness: 60,
                pace: 99,
                overall: 99,
                points: 10,
            },
            driver_2: Driver {
                driver_name: DriverName::CarlosSainz,
                expierence: 3,
                race_craft: 67,
                awareness: 80,
                pace: 85,
                overall: 99,
                points: 20,
            },
            reserve_driver: Driver {
                driver_name: DriverName::LewisHamilton,
                expierence: 15,
                race_craft: 90,
                awareness: 98,
                pace: 97,
                overall: 99,
                points: 30,
            },
            budget: BUDGET,
            points: Default::default()
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
            Driver {
                driver_name: DriverName::CharlesLeclerc,
                expierence: 4,
                race_craft: 75,
                awareness: 60,
                pace: 99,
                overall: 99,
                points: 10,
            },
            Driver {
                driver_name: DriverName::CarlosSainz,
                expierence: 3,
                race_craft: 67,
                awareness: 80,
                pace: 85,
                overall: 99,
                points: 20,
            },
            Driver {
                driver_name: DriverName::LewisHamilton,
                expierence: 15,
                race_craft: 90,
                awareness: 98,
                pace: 97,
                overall: 99,
                points: 30,
            },
        );

        assert_eq!(expected_team, team);
    }
}
