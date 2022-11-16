use super::race::Race;

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

    // pub fn calculate_season_points(mut races: [Race; 22]) {
    //     for race in 0..races.len() {
    // overall_points += races[race].race_results[race].race_points;
    // races[race].race_results[driver].season_points = overall_points;
    //     }
    // }
}

#[cfg(test)]
mod season_should {
    use rand::random;

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
