use super::{team_name::TeamName, driver::Driver};

const BUDGET: u64 = 150000000;

#[derive(Debug, PartialEq)]
pub struct Team {
    team_name: TeamName,
    driver_1: Driver,
    driver_2: Driver,
    reserve_driver: Driver,
    budget: u64,
}

impl Team {
    fn new(
        team_name: TeamName,
        driver_1: Driver,
        driver_2: Driver,
        reserve_driver: Driver,
    ) -> Self {
        Self {
            team_name,
            driver_1,
            driver_2,
            reserve_driver,
            budget: BUDGET,
        }
    }
}

#[cfg(test)]
mod team_name_should {
    use crate::models::driver_name::DriverName;
    use super::*;

    #[test]
    fn create_a_team() {
        let expected_team = Team {
            team_name: TeamName::Haas,
            driver_1: Driver{ driver_name: DriverName::CharlesLeclerc, expierence: 4, race_craft: 75, awareness: 60, pace: 99, overall: 99 },
            driver_2: Driver{ driver_name: DriverName::CarlosSainz, expierence: 3, race_craft: 67, awareness: 80, pace: 85, overall: 99 },
            reserve_driver: Driver{ driver_name: DriverName::LewisHamilton, expierence: 15, race_craft: 90, awareness: 98, pace: 97, overall: 99 },
            budget: BUDGET,
        };

        let team = Team::new(
            TeamName::Haas,
            Driver{ driver_name: DriverName::CharlesLeclerc, expierence: 4, race_craft: 75, awareness: 60, pace: 99, overall: 99 },
            Driver{ driver_name: DriverName::CarlosSainz, expierence: 3, race_craft: 67, awareness: 80, pace: 85, overall: 99 },
            Driver{ driver_name: DriverName::LewisHamilton, expierence: 15, race_craft: 90, awareness: 98, pace: 97, overall: 99 },
        );

        assert_eq!(expected_team, team);
    }
}
