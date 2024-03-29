use crate::models::{
    races::race_grid::RaceGrid, seasons::season_points::SeasonPoints, teams::team_seeds::TeamSeeds,
};

pub const NUMBER_OF_RACES_IN_A_SEASON: usize = 10;

#[derive(Clone, Copy)]
pub struct Season {
    pub races: [RaceGrid; NUMBER_OF_RACES_IN_A_SEASON],
}

impl Default for Season {
    fn default() -> Self {
        let team_seeds = TeamSeeds::default().team_seeds;
        Self {
            races: [
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
                RaceGrid::new(team_seeds),
            ],
        }
    }
}

impl Season {
    // TODO test
    pub fn calculate_race_results(&mut self) {
        let mut race_number = 0;

        for race in &mut self.races {
            race_number += 1;

            race.display_race_information(race_number);

            race.calculate_driver_race_chances();
            let scoring_drivers = race.race_result_order();
            race.assign_points(scoring_drivers, race_number);

            let race_result = race.get_drivers_on_the_race_grid();
            let ordered_race_result = RaceGrid::order_race_result(race_result);
            RaceGrid::display_race_results(ordered_race_result);
        }

        let mut season_result = SeasonPoints::update_driver_season_points(self);
        season_result.order_season_results();

        println!("{}", season_result);
    }
}

#[cfg(test)]
mod season_should {
    use crate::models::seasons::season::{Season, NUMBER_OF_RACES_IN_A_SEASON};

    #[test]
    fn new_grid() {
        // Given
        let race_grid = Season::default();

        // Then
        assert_eq!(NUMBER_OF_RACES_IN_A_SEASON, race_grid.races.len())
    }
}
