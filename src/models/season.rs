use super::{driver::Driver, race::Race, team::Team, team_name::TeamName};

#[derive(Clone, Copy)]
pub struct Season {
    races: [Race; 22],
    pub drivers_championship: [Driver; 22],
    pub constructors_championship: [Team; 11],
}

impl Season {
    pub fn new(races: [Race; 22]) -> Season {
        Self {
            races,
            drivers_championship: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            constructors_championship: Default::default(),
        }
    }

    pub fn reset_season_points(&mut self) {
        for race in 0..self.races.len() {
            for driver in 0..self.races.len() {
                self.races[race].race_results[driver].season_points = 0;
                self.races[race].race_results[driver].team.season_points = 0;
            }
        }
    }

    pub fn calculate_driver_season_points(&mut self) {
        //TODO I believe there is a bug in here whereby driver results get lost and are not added up for the driver standings
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

        let last_race = self.races.len() - 1;
        self.drivers_championship = self.races[last_race].race_results;
    }

    pub fn order_driver_standings(&mut self) {
        self.drivers_championship
            .sort_by(|d1, d2| d1.season_points.partial_cmp(&d2.season_points).unwrap());

        self.drivers_championship.reverse();
    }

    pub fn calculate_constructor_season_points(&mut self) {
        self.calculate_driver_season_points();

        for driver in self.drivers_championship {
            match driver.team.name {
                TeamName::Ferrari => {
                    self.constructors_championship[0].name = TeamName::Ferrari;
                    self.constructors_championship[0].season_points += driver.season_points
                }
                TeamName::Mercedes => {
                    self.constructors_championship[1].name = TeamName::Mercedes;
                    self.constructors_championship[1].season_points += driver.season_points
                }
                TeamName::RedBull => {
                    self.constructors_championship[2].name = TeamName::RedBull;
                    self.constructors_championship[2].season_points += driver.season_points
                }
                TeamName::Alpine => {
                    self.constructors_championship[3].name = TeamName::Alpine;
                    self.constructors_championship[3].season_points += driver.season_points
                }
                TeamName::Mclaren => {
                    self.constructors_championship[4].name = TeamName::Mclaren;
                    self.constructors_championship[4].season_points += driver.season_points
                }
                TeamName::Haas => {
                    self.constructors_championship[5].name = TeamName::Haas;
                    self.constructors_championship[5].season_points += driver.season_points
                }
                TeamName::AlphaRomeo => {
                    self.constructors_championship[6].name = TeamName::AlphaRomeo;
                    self.constructors_championship[6].season_points += driver.season_points
                }
                TeamName::AlphaTauri => {
                    self.constructors_championship[7].name = TeamName::AlphaTauri;
                    self.constructors_championship[7].season_points += driver.season_points
                }
                TeamName::AstonMartin => {
                    self.constructors_championship[8].name = TeamName::AstonMartin;
                    self.constructors_championship[8].season_points += driver.season_points
                }
                TeamName::Williams => {
                    self.constructors_championship[9].name = TeamName::Williams;
                    self.constructors_championship[9].season_points += driver.season_points
                }
                // the 11th team
                TeamName::Minardi => {
                    self.constructors_championship[10].name = TeamName::Minardi;
                    self.constructors_championship[10].season_points += driver.season_points
                }
                TeamName::ToroRosso => {
                    self.constructors_championship[10].name = TeamName::ToroRosso;
                    self.constructors_championship[10].season_points += driver.season_points
                }
            }
        }
    }

    pub fn order_constructor_standings(&mut self) {
        self.constructors_championship
            .sort_by(|t1, t2| t1.season_points.partial_cmp(&t2.season_points).unwrap());

        self.constructors_championship.reverse();
    }
}

#[cfg(test)]
mod season_should {
    use super::*;
    use crate::models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName};
    use rand::random;
    use rstest::rstest;

    #[test]
    fn be_able_to_reset_season_points() {
        let mut season = Season::new(season_test_fixture().races);

        season.reset_season_points();

        for race in 0..season.races.len() {
            for driver in 0..season.races.len() {
                assert_eq!(0, season.races[race].race_results[driver].season_points);
                assert_eq!(
                    0,
                    season.races[race].race_results[driver].team.season_points
                );
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
    fn be_able_to_calculate_driver_season_points(
        #[case] expected_points: u16,
        #[case] race: usize,
        #[case] driver: usize,
    ) {
        let mut season = Season::new(season_test_fixture().races);

        season.calculate_driver_season_points();

        assert_eq!(
            expected_points,
            season.races[race].race_results[driver].season_points
        );
        assert_eq!(220, season.drivers_championship[0].season_points);
        assert_eq!(220, season.drivers_championship[1].season_points);
    }

    #[test]
    fn be_able_to_order_driver_standings() {
        let mut season = Season::new(season_test_fixture().races);
        season.drivers_championship[6].season_points = 600;
        season.drivers_championship[9].season_points = 400;
        season.drivers_championship[1].season_points = 300;
        season.drivers_championship[0].season_points = 200;

        season.order_driver_standings();

        assert_eq!(600, season.drivers_championship[0].season_points);
        assert_eq!(400, season.drivers_championship[1].season_points);
        assert_eq!(300, season.drivers_championship[2].season_points);
        assert_eq!(200, season.drivers_championship[3].season_points);
    }

    #[test]
    fn be_able_to_calculate_constructor_season_points() {
        let mut season = Season::new(season_test_fixture().races);

        season.calculate_constructor_season_points();

        assert_eq!(TeamName::Ferrari, season.constructors_championship[0].name);
        assert_eq!(TeamName::Mercedes, season.constructors_championship[1].name);
        assert_eq!(TeamName::RedBull, season.constructors_championship[2].name);
        assert_eq!(3960, season.constructors_championship[0].season_points);
        assert_eq!(440, season.constructors_championship[1].season_points);
        assert_eq!(440, season.constructors_championship[2].season_points);
    }

    #[test]
    fn be_able_to_order_constructor_standings() {
        let mut season = Season::new(season_test_fixture().races);
        // Ferrari
        season.constructors_championship[0].name = TeamName::Ferrari;
        season.constructors_championship[0].season_points = 1000;
        // Redbull
        season.constructors_championship[10].name = TeamName::RedBull;
        season.constructors_championship[10].season_points = 500;
        // Mercedes
        season.constructors_championship[9].name = TeamName::Mercedes;
        season.constructors_championship[9].season_points = 850;

        season.order_constructor_standings();

        assert_eq!(TeamName::Ferrari, season.constructors_championship[0].name);
        assert_eq!(1000, season.constructors_championship[0].season_points);
        assert_eq!(TeamName::Mercedes, season.constructors_championship[1].name);
        assert_eq!(850, season.constructors_championship[1].season_points);
        assert_eq!(TeamName::RedBull, season.constructors_championship[2].name);
        assert_eq!(500, season.constructors_championship[2].season_points);
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
            drivers_championship: Default::default(),
            constructors_championship: Default::default(),
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
                driver_test_fixture_2(),
                driver_test_fixture_2(),
                driver_test_fixture_3(),
                driver_test_fixture_3(),
            ],
        }
    }

    fn driver_test_fixture() -> Driver {
        let mut driver = Driver::new(DriverName::MaxVerstappen, team_test_fixture(), [1, 2, 3, 4]);
        driver.season_points = 10;
        driver
    }

    fn driver_test_fixture_2() -> Driver {
        let mut driver = Driver::new(
            DriverName::LewisHamilton,
            team_test_fixture_2(),
            [1, 2, 3, 4],
        );
        driver.season_points = 10;
        driver
    }

    fn driver_test_fixture_3() -> Driver {
        let mut driver = Driver::new(
            DriverName::CharlesLeclerc,
            team_test_fixture_3(),
            [1, 2, 3, 4],
        );
        driver.season_points = 10;
        driver
    }

    fn team_test_fixture() -> Team {
        let mut team = Team::new(TeamName::Ferrari, [1, 2, 3, 4]);
        team.season_points = 10;
        team
    }

    fn team_test_fixture_2() -> Team {
        let mut team = Team::new(TeamName::Mercedes, [1, 2, 3, 4]);
        team.season_points = 10;
        team
    }

    fn team_test_fixture_3() -> Team {
        let mut team = Team::new(TeamName::RedBull, [1, 2, 3, 4]);
        team.season_points = 10;
        team
    }
}
