use super::{driver::Driver, points::Points, team::Team};

pub struct Race {
    teams: [Team; 10],
    race_results: [Driver; 20],
    points: Points,
}

impl Race {
    pub fn calculate_race_result(&mut self, seed: u64) {
        let mut index = 0;

        for mut team in self.teams {
            team.calculate_race_chances(seed);

            self.race_results[index] = team.driver_1;
            if index <= 9 {
                team.driver_1
                    .add_points(self.points.points_allocation[index] as u16);
            }
            index += 1;

            self.race_results[index] = team.driver_2;
            if index <= 9 {
                team.driver_2
                    .add_points(self.points.points_allocation[index] as u16);
            }
            index += 1;

            team.calculate_points();
        }

        self.race_results
            .sort_by(|d1, d2| d1.race_chances.partial_cmp(&d2.race_chances).unwrap());

        self.race_results.reverse()
    }

    fn assign_points(&mut self) {
        for index in 0..10 {
            self.race_results[index].points = self.points.points_allocation[index] as u16;
        }
    }
}

#[cfg(test)]
mod race_should {
    use super::*;
    use crate::models::{car::Car, driver_name::DriverName, team_name::TeamName};

    #[test]
    fn calculate_the_race_results() {
        // created a test to show that the race result
        // is based on the race chance order for each
        // driver in each team and will place based on their overall
        // package as driver and car
        let race_results: [Driver; 20] = [
            race_chances_driver_test_fixture(209.2088),
            race_chances_driver_test_fixture(203.67418),
            race_chances_driver_test_fixture(194.81879),
            race_chances_driver_test_fixture(188.85162),
            race_chances_driver_test_fixture(188.85162),
            race_chances_driver_test_fixture(183.74953),
            race_chances_driver_test_fixture(176.7328),
            race_chances_driver_test_fixture(174.89413),
            race_chances_driver_test_fixture(171.6833),
            race_chances_driver_test_fixture(161.58429),
            race_chances_driver_test_fixture(151.64871),
            race_chances_driver_test_fixture(143.40605),
            race_chances_driver_test_fixture(138.36562),
            race_chances_driver_test_fixture(136.15176),
            race_chances_driver_test_fixture(134.31694),
            race_chances_driver_test_fixture(133.30704),
            race_chances_driver_test_fixture(110.69249),
            race_chances_driver_test_fixture(83.01936),
            race_chances_driver_test_fixture(82.81194),
            race_chances_driver_test_fixture(60.594105),
        ];
        let mut race = Race {
            teams: [
                team_test_fixture(99, 80, 90),
                team_test_fixture(60, 20, 40),
                team_test_fixture(67, 72, 70),
                team_test_fixture(78, 80, 80),
                team_test_fixture(89, 92, 95),
                team_test_fixture(25, 32, 50),
                team_test_fixture(69, 77, 56),
                team_test_fixture(84, 95, 92),
                team_test_fixture(45, 54, 78),
                team_test_fixture(78, 87, 88),
            ],
            race_results: [
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
            ],
            points: Points::default(),
        };

        race.calculate_race_result(2022);

        for index in 0..20 {
            assert_eq!(
                race_results[index].race_chances,
                race.race_results[index].race_chances
            );
        }

        assert_ne!(0, race.teams[2].points);
        assert_ne!(0, race.teams[2].driver_1.points);
    }

    #[test]
    fn assign_points_based_on_the_race_finish_order() {
        let mut race = Race {
            teams: [
                team_test_fixture(99, 80, 90),
                team_test_fixture(60, 20, 40),
                team_test_fixture(67, 72, 70),
                team_test_fixture(78, 80, 80),
                team_test_fixture(89, 92, 95),
                team_test_fixture(25, 32, 50),
                team_test_fixture(69, 77, 56),
                team_test_fixture(84, 95, 92),
                team_test_fixture(45, 54, 78),
                team_test_fixture(78, 87, 88),
            ],
            race_results: [
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
            ],
            points: Points::default(),
        };

        race.assign_points();

        assert_eq!(25, race.race_results[0].points);
        assert_eq!(18, race.race_results[1].points);
        assert_eq!(15, race.race_results[2].points);
        assert_eq!(12, race.race_results[3].points);
        assert_eq!(10, race.race_results[4].points);
        assert_eq!(8, race.race_results[5].points);
        assert_eq!(6, race.race_results[6].points);
        assert_eq!(4, race.race_results[7].points);
        assert_eq!(2, race.race_results[8].points);
        assert_eq!(1, race.race_results[9].points);
        assert_eq!(0, race.race_results[10].points);
        assert_eq!(0, race.race_results[11].points);
        assert_eq!(0, race.race_results[12].points);
        assert_eq!(0, race.race_results[13].points);
        assert_eq!(0, race.race_results[14].points);
        assert_eq!(0, race.race_results[15].points);
        assert_eq!(0, race.race_results[16].points);
        assert_eq!(0, race.race_results[17].points);
        assert_eq!(0, race.race_results[18].points);
        assert_eq!(0, race.race_results[19].points);
    }

    fn team_test_fixture(
        overall_stat_for_driver_1: u32,
        overall_stat_for_driver_2: u32,
        overall_stat_for_car: u32,
    ) -> Team {
        Team {
            team_name: TeamName::Haas,
            car: Car {
                aero: Default::default(),
                engine: Default::default(),
                reliability: Default::default(),
                tire_management: Default::default(),
                overall: overall_stat_for_car,
            },
            driver_1: overall_driver_test_fixture(overall_stat_for_driver_1),
            driver_2: overall_driver_test_fixture(overall_stat_for_driver_2),
            points: Default::default(),
        }
    }

    fn overall_driver_test_fixture(overall: u32) -> Driver {
        Driver {
            driver_name: DriverName::LewisHamilton,
            expierence: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall,
            race_chances: Default::default(),
            points: Default::default(),
        }
    }

    fn race_chances_driver_test_fixture(race_chances: f32) -> Driver {
        Driver {
            driver_name: DriverName::LewisHamilton,
            expierence: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: Default::default(),
            race_chances,
            points: Default::default(),
        }
    }
}
