use rand::random;

use super::{driver::Driver, points::Points, race_name::RaceName};

pub struct Race {
    pub name: RaceName,
    pub race_results: [Driver; 22],
}

impl Race {
    pub fn calculate_race_chances(&mut self, seed: u64) {
        for index in 0..22 {
            let mut driver = self.race_results[index];
            // TODO This will generate the same chance for each driver
            driver.calculate_race_chance(seed);
            self.race_results[index] = driver;
        }
    }

    pub fn sort_racing_result_order(&mut self) {
        self.race_results
            .sort_by(|d1, d2| d1.race_chance.partial_cmp(&d2.race_chance).unwrap());

        self.race_results.reverse();
    }

    pub fn assign_points(&mut self) {
        let points = Points::default();

        for index in 0..10 {
            self.race_results[index].add_points(points.points_allocation[index] as u16);
        }
    }
}

#[cfg(test)]
mod race_should {
    use super::*;
    use crate::models::{car::Car, driver_name::DriverName, team::Team, team_name::TeamName};

    #[test]
    fn calculate_race_chances() {
        let expected_race_results: [Driver; 22] = [
            race_chances_driver_test_fixture(22.138498),
            race_chances_driver_test_fixture(44.276997),
            race_chances_driver_test_fixture(66.4155),
            race_chances_driver_test_fixture(110.69249),
            race_chances_driver_test_fixture(110.69249),
            race_chances_driver_test_fixture(88.55399),
            race_chances_driver_test_fixture(110.69249),
            race_chances_driver_test_fixture(132.831),
            race_chances_driver_test_fixture(154.96948),
            race_chances_driver_test_fixture(177.10799),
            race_chances_driver_test_fixture(199.24648),
            race_chances_driver_test_fixture(219.17113),
            race_chances_driver_test_fixture(2.2138498),
            race_chances_driver_test_fixture(4.4276996),
            race_chances_driver_test_fixture(6.641549),
            race_chances_driver_test_fixture(8.855399),
            race_chances_driver_test_fixture(11.069249),
            race_chances_driver_test_fixture(13.283098),
            race_chances_driver_test_fixture(15.496948),
            race_chances_driver_test_fixture(17.710798),
            race_chances_driver_test_fixture(17.710798),
            race_chances_driver_test_fixture(17.710798),
        ];
        let mut race = Race {
            name: RaceName::Donnington,
            race_results: [
                overall_driver_test_fixture(10, 10),
                overall_driver_test_fixture(20, 20),
                overall_driver_test_fixture(30, 30),
                overall_driver_test_fixture(90, 10),
                overall_driver_test_fixture(10, 90),
                overall_driver_test_fixture(40, 40),
                overall_driver_test_fixture(50, 50),
                overall_driver_test_fixture(60, 60),
                overall_driver_test_fixture(70, 70),
                overall_driver_test_fixture(80, 80),
                overall_driver_test_fixture(90, 90),
                overall_driver_test_fixture(99, 99),
                overall_driver_test_fixture(1, 1),
                overall_driver_test_fixture(2, 2),
                overall_driver_test_fixture(3, 3),
                overall_driver_test_fixture(4, 4),
                overall_driver_test_fixture(5, 5),
                overall_driver_test_fixture(6, 6),
                overall_driver_test_fixture(7, 7),
                overall_driver_test_fixture(8, 8),
                overall_driver_test_fixture(8, 8),
                overall_driver_test_fixture(8, 8),
            ],
        };

        race.calculate_race_chances(2022);

        for index in 0..22 {
            assert_eq!(
                expected_race_results[index].race_chance,
                race.race_results[index].race_chance
            );
        }
    }

    #[test]
    fn sort_racing_result_order() {
        let expected_race_results: [Driver; 22] = [
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
            race_chances_driver_test_fixture(60.594105),
            race_chances_driver_test_fixture(60.594105),
        ];
        let mut race = Race {
            name: RaceName::Donnington,
            race_results: [
                race_chances_driver_test_fixture(188.85162),
                race_chances_driver_test_fixture(194.81879),
                race_chances_driver_test_fixture(151.64871),
                race_chances_driver_test_fixture(183.74953),
                race_chances_driver_test_fixture(176.7328),
                race_chances_driver_test_fixture(136.15176),
                race_chances_driver_test_fixture(209.2088),
                race_chances_driver_test_fixture(188.85162),
                race_chances_driver_test_fixture(171.6833),
                race_chances_driver_test_fixture(161.58429),
                race_chances_driver_test_fixture(174.89413),
                race_chances_driver_test_fixture(203.67418),
                race_chances_driver_test_fixture(83.01936),
                race_chances_driver_test_fixture(143.40605),
                race_chances_driver_test_fixture(134.31694),
                race_chances_driver_test_fixture(60.594105),
                race_chances_driver_test_fixture(110.69249),
                race_chances_driver_test_fixture(138.36562),
                race_chances_driver_test_fixture(133.30704),
                race_chances_driver_test_fixture(82.81194),
                race_chances_driver_test_fixture(60.594105),
                race_chances_driver_test_fixture(60.594105),
            ],
        };

        race.sort_racing_result_order();

        for index in 0..22 {
            assert_eq!(
                expected_race_results[index].race_chance,
                race.race_results[index].race_chance
            );
        }
    }

    #[test]
    fn assign_points_based_on_the_race_finish_order() {
        let mut race = Race {
            name: RaceName::Donnington,
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
                race_chances_driver_test_fixture(Default::default()),
                race_chances_driver_test_fixture(Default::default()),
            ],
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
        assert_eq!(0, race.race_results[20].points);
        assert_eq!(0, race.race_results[21].points);
        // TODO need to sort out different drivers being part of the same team instance
        assert_eq!(25, race.race_results[0].team.points);
        assert_eq!(18, race.race_results[1].team.points);
        assert_eq!(15, race.race_results[2].team.points);
        assert_eq!(12, race.race_results[3].team.points);
        assert_eq!(10, race.race_results[4].team.points);
        assert_eq!(8, race.race_results[5].team.points);
        assert_eq!(6, race.race_results[6].team.points);
        assert_eq!(4, race.race_results[7].team.points);
        assert_eq!(2, race.race_results[8].team.points);
        assert_eq!(1, race.race_results[9].team.points);
        assert_eq!(0, race.race_results[10].team.points);
        assert_eq!(0, race.race_results[11].team.points);
        assert_eq!(0, race.race_results[12].team.points);
        assert_eq!(0, race.race_results[13].team.points);
        assert_eq!(0, race.race_results[14].team.points);
        assert_eq!(0, race.race_results[15].team.points);
        assert_eq!(0, race.race_results[16].team.points);
        assert_eq!(0, race.race_results[17].team.points);
        assert_eq!(0, race.race_results[18].team.points);
        assert_eq!(0, race.race_results[19].team.points);
        assert_eq!(0, race.race_results[20].team.points);
        assert_eq!(0, race.race_results[21].team.points);
    }

    fn overall_driver_test_fixture(driver_overall: u32, car_overall: u32) -> Driver {
        Driver {
            name: DriverName::LewisHamilton,
            team: team_test_fixture(car_overall),
            experience: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: driver_overall,
            race_chance: Default::default(),
            points: Default::default(),
        }
    }

    fn team_test_fixture(car_overall: u32) -> Team {
        Team {
            name: TeamName::Haas,
            car: Car {
                aero: Default::default(),
                engine: Default::default(),
                reliability: Default::default(),
                tire_management: Default::default(),
                overall: car_overall,
            },
            points: Default::default(),
        }
    }

    fn race_chances_driver_test_fixture(race_chances: f32) -> Driver {
        Driver {
            name: DriverName::LewisHamilton,
            team: team_test_fixture(Default::default()),
            experience: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: Default::default(),
            race_chance: race_chances,
            points: Default::default(),
        }
    }
}
