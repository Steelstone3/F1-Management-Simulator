use super::{
    names::{driver_names::DriverName, team_names::TeamName},
    points::Points,
    ratings::driver_rating::DriverRating,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub team_name: TeamName,
    pub driver_rating: DriverRating,
    pub points: Points,
}
impl Driver {
    pub fn new(driver_name: DriverName, team_name: TeamName, seeds: [u64; 5]) -> Driver {
        Self {
            driver_name,
            team_name,
            driver_rating: DriverRating::new(seeds),
            points: Points::default(),
        }
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;

    #[test]
    fn create_new_driver() {
        // Given
        let expected_driver = Driver {
            driver_name: DriverName::LewisHamilton,
            team_name: TeamName::Mercedes,
            driver_rating: DriverRating {
                awareness: 90,
                consistency: 64,
                experience: 81,
                race_craft: 84,
                pace: 66,
                overall: 77,
                overall_race_chance: Default::default(),
            },
            points: Points::default(),
        };

        // When
        let driver = Driver::new(
            DriverName::LewisHamilton,
            TeamName::Mercedes,
            [1, 2, 3, 4, 5],
        );

        // Then
        assert_eq!(expected_driver, driver);
    }
}
