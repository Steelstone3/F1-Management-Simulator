use super::{driver::Driver, race::Race};

pub struct Season {
    races: [Race; 22],
}

impl Season {
    pub fn new(races: [Race; 22]) -> Season {
        Self { races }
    }

    pub fn reset_season_points(&mut self) {
        for race in 0..self.races.len() {
            for driver in 0..self.races.len() {
                self.races[race].race_results[driver].season_points = 0;
            }
        }
    }

    pub fn calculate_driver_season_points(&mut self) -> [Driver; 22] {
        for race in 0..self.races.len() {
            for driver in 0..self.races.len() {
                if race == 0 {
                    self.races[race].race_results[driver].season_points +=
                        self.races[race].race_results[driver].race_points;
                } else {
                    self.races[race].race_results[driver].season_points +=
                        self.races[race].race_results[driver].race_points
                            + self.races[race - 1].race_results[driver].season_points;
                }
            }
        }

        self.races[self.races.len() - 1].race_results
    }

    pub fn order_driver_standings(&mut self, mut season_result: [Driver; 22]) -> [Driver; 22] {
        // let mut ordered_driver_standings = self.races[21].race_results;

        season_result.sort_by(|d1, d2| d1.season_points.partial_cmp(&d2.season_points).unwrap());

        season_result.reverse();

        season_result
    }
}

#[cfg(test)]
mod season_should {
    use rand::random;
    use rstest::rstest;

    use crate::models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName};

    use super::*;

    #[test]
    fn be_able_to_reset_season_points() {
        let mut season = Season::new(season_test_fixture().races);

        season.reset_season_points();

        for race in 0..season.races.len() {
            for driver in 0..season.races.len() {
                assert_eq!(0, season.races[race].race_results[driver].season_points);
            }
        }
    }

    #[rstest]
    #[case(10, 0, 0)]
    #[case(20, 1, 0)]
    #[case(30, 2, 0)]
    #[case(10, 0, 1)]
    #[case(10, 0, 2)]
    #[case(10, 0, 3)]
    #[case(40, 3, 3)]
    fn be_able_to_calculate_season_points(
        #[case] expected_points: u16,
        #[case] race: usize,
        #[case] driver: usize,
    ) {
        let mut season = Season::new(season_test_fixture().races);

        let last_result = season.calculate_driver_season_points();

        assert_eq!(
            expected_points,
            season.races[race].race_results[driver].season_points
        );
        assert_eq!(220, last_result[0].season_points);
    }

    #[test]
    fn be_able_to_order_driver_standings() {
        let mut season = Season::new(season_test_fixture().races);
        let last_race = season.races.len() - 1;
        season.races[last_race].race_results[6].season_points = 600;
        season.races[last_race].race_results[9].season_points = 400;
        season.races[last_race].race_results[1].season_points = 300;
        season.races[last_race].race_results[0].season_points = 200;

        let ordered_driver_standing =
            season.order_driver_standings(season.races[last_race].race_results);

        assert_eq!(600, ordered_driver_standing[0].season_points);
        assert_eq!(400, ordered_driver_standing[1].season_points);
        assert_eq!(300, ordered_driver_standing[2].season_points);
        assert_eq!(200, ordered_driver_standing[3].season_points);
    }

    fn season_test_fixture() -> Season {
        Season {
            races: [
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
                race_test_fixture(),
            ],
        }
    }

    fn race_test_fixture() -> Race {
        Race {
            name: random(),
            race_results: [
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
                driver_test_fixture(),
            ],
        }
    }

    fn driver_test_fixture() -> Driver {
        let mut driver = Driver::new(DriverName::MaxVerstappen, team_test_fixture(), [1, 2, 3, 4]);
        driver.add_points(10);
        driver
    }

    fn team_test_fixture() -> Team {
        Team::new(TeamName::Ferrari, [1, 2, 3, 4])
    }
}
