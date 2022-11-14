use super::{driver_name::DriverName, team_name::TeamName};

pub struct Team {
    team_name: TeamName,
    driver_1: DriverName,
    driver_2: DriverName,
    reserve_driver: DriverName,
    budget: u64,
}

impl Team {
    fn new(
        team_name: TeamName,
        driver_1: DriverName,
        driver_2: DriverName,
        reserve_driver: DriverName,
    ) -> Self {
        Self {
            team_name,
            driver_1,
            driver_2,
            reserve_driver,
            budget: 150000000,
        }
    }
}

#[cfg(test)]
mod team_name_should {
    use crate::models::{driver_name::DriverName, team_name::TeamName};

    use super::Team;

    #[test]
    fn create_a_team() {
        let expected_team = Team {
            team_name: TeamName::Haas,
            driver_1: DriverName::CharlesLeclerc,
            driver_2: DriverName::CarlosSainz,
            reserve_driver: DriverName::LewisHamilton,
            budget: 150000000,
        };

        let team = Team::new(
            TeamName::Haas,
            DriverName::CharlesLeclerc,
            DriverName::CarlosSainz,
            DriverName::LewisHamilton,
        );

        assert_eq!(
            expected_team.team_name.to_string(),
            team.team_name.to_string()
        );
        assert_eq!(
            expected_team.driver_1.to_string(),
            team.driver_1.to_string()
        );
        assert_eq!(
            expected_team.driver_2.to_string(),
            team.driver_2.to_string()
        );
        assert_eq!(
            expected_team.reserve_driver.to_string(),
            team.reserve_driver.to_string()
        );
        assert_eq!(expected_team.budget, team.budget);
    }
}
