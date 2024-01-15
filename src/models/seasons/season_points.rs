use crate::models::races::race_grid::TEAMS_ON_THE_RACE_GRID;

use super::{
    season::{Season, NUMBER_OF_RACES_IN_A_SEASON},
    season_result::SeasonResult,
};

pub const RACE_POSIITIONS_THAT_ALLOCATE_POINTS: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SeasonPoints {
    pub season_points: u32,
}

impl SeasonPoints {
    // TODO test
    pub fn update_driver_season_points(season: &Season) -> SeasonResult {
        let mut season_result = SeasonResult::new(season.races[9].teams);

        for race_number in 0..NUMBER_OF_RACES_IN_A_SEASON {
            for team_result_index in 0..TEAMS_ON_THE_RACE_GRID {
                for team in season.races[race_number].teams {
                    if season_result.results[team_result_index].team_name == team.team_name {
                        season_result.add_season_points(team_result_index, team);
                    }
                }
            }
        }

        season_result
    }
}

#[cfg(test)]
mod season_points_should {

    // #[rstest]
    // #[case([0,0,0,0,0,0,0,0,0,0], 0)]
    // #[case([10,0,0,0,0,0,0,0,0,0], 10)]
    // #[case([0,0,0,0,0,15,0,0,0,0], 15)]
    // #[case([10,8,6,8,15,25,18,25,12,8], 135)]
    // #[case([25,25,25,25,25,25,25,25,25,25], 250)]
    // fn calculate_the_points_achieved_over_the_season(
    //     #[case] season_points: [u32; RACE_POSIITIONS_THAT_ALLOCATE_POINTS],
    //     #[case] expected_season_points: u32,
    // ) {
    //     // Given
    //     let points = SeasonPoints {
    //         season_points,
    //         ..Default::default()
    //     };

    //     // When
    //     let season_points = points.calculate_season_points();

    //     // Then
    //     assert_eq!(expected_season_points, season_points);
    // }
}
