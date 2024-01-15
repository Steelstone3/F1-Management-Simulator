use crate::models::{
    drivers::{
        driver::Driver,
        driver_name::{self, DriverName},
    },
    races::race_grid::{TEAMS_ON_THE_RACE_GRID, DRIVERS_ON_THE_RACE_GRID},
    teams::{team::Team, team_name::TeamName},
};
use std::fmt::Display;

use super::season::{Season, NUMBER_OF_RACES_IN_A_SEASON};

// TODO aggrigate result holder
pub struct SeasonResult {
    teams: [Team; TEAMS_ON_THE_RACE_GRID],
}

impl Display for SeasonResult {
    fn fmt(&self, formatting: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatting, "Something")
    }
}

impl SeasonResult {
    pub fn new(teams: [Team; TEAMS_ON_THE_RACE_GRID]) -> Self {
        Self { teams }
    }

    pub fn aggregate_drivers_from_races(&self, season: Season) -> [Driver; DRIVERS_ON_THE_RACE_GRID] {
        let mut drivers_at_races = vec![];

        for race in &season.races {
            for team in race.teams {
                drivers_at_races.push(team.driver_1);
                drivers_at_races.push(team.driver_2);
            }
        }

        drivers_at_races.try_into().unwrap()
    }

    pub fn aggregate_driver_season_points(
        &mut self,
        drivers_at_races: [Driver; DRIVERS_ON_THE_RACE_GRID],
    ) {
        for driver in drivers_at_races{
            let team_index = driver.find_team(&self.teams);
            self.teams[team_index].add_season_points(drivers_at_races);
        }
    }
}
