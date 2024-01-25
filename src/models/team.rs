use super::{
    driver::Driver,
    names::{driver_names::DriverName, team_names::TeamName},
    ratings::{car_rating::CarRating, team_rating::TeamRating},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Team {
    pub team_name: TeamName,
    pub team_rating: TeamRating,
    pub car_rating: CarRating,
    pub driver_1: Driver,
    pub driver_2: Driver,
}

impl Team {
    pub fn new(
        team_name: TeamName,
        driver_1_name: DriverName,
        driver_2_name: DriverName,
        seeds: [u64; 19],
    ) -> Self {
        let team_rating_seeds = [seeds[0], seeds[1], seeds[2], seeds[3], seeds[4]];
        let car_rating_seeds = [seeds[5], seeds[6], seeds[7], seeds[8]];
        let driver_1_rating_seeds = [seeds[9], seeds[10], seeds[11], seeds[12], seeds[13]];
        let driver_2_rating_seeds = [seeds[14], seeds[15], seeds[16], seeds[17], seeds[18]];

        Self {
            team_name: TeamName::Mercedes,
            team_rating: TeamRating::new(team_rating_seeds),
            car_rating: CarRating::new(car_rating_seeds),
            driver_1: Driver::new(driver_1_name, team_name, driver_1_rating_seeds),
            driver_2: Driver::new(driver_2_name, team_name, driver_2_rating_seeds),
        }
    }
}

#[cfg(test)]
mod team_should {
    use super::*;
    use crate::models::{points::Points, ratings::driver_rating::DriverRating};

    #[test]
    fn create_new_team() {
        // Given
        let seeds = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let expected_team = Team {
            team_name: TeamName::Mercedes,
            team_rating: TeamRating {
                car_development: 90,
                car_repairs: 64,
                car_setup: 81,
                pitstops: 84,
                team_management: 66,
                overall: 77,
            },
            car_rating: CarRating {
                aero: 66,
                engine: 70,
                reliability: 67,
                tire_management: 81,
                overall: 71,
            },
            driver_1: Driver {
                driver_name: DriverName::LewisHamilton,
                team_name: TeamName::Mercedes,
                driver_rating: DriverRating {
                    awareness: 56,
                    consistency: 61,
                    experience: 70,
                    race_craft: 78,
                    pace: 91,
                    overall: 71,
                    overall_race_chance: 0,
                },
                points: Points::default(),
            },
            driver_2: Driver {
                driver_name: DriverName::GeorgeRussell,
                team_name: TeamName::Mercedes,
                driver_rating: DriverRating {
                    awareness: 90,
                    consistency: 95,
                    experience: 82,
                    race_craft: 65,
                    pace: 88,
                    overall: 84,
                    overall_race_chance: 0,
                },
                points: Points::default(),
            },
        };

        // When
        let team = Team::new(
            TeamName::Mercedes,
            DriverName::LewisHamilton,
            DriverName::GeorgeRussell,
            seeds,
        );

        // Then
        assert_eq!(expected_team, team)
    }
}
