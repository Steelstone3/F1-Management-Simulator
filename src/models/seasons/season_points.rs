use crate::models::races::race_points::RacePoints;

use super::season::NUMBER_OF_RACES_IN_A_SEASON;

pub const RACE_POSIITIONS_THAT_ALLOCATE_POINTS: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SeasonPoints {
    pub season_points: u32,
}

impl SeasonPoints {
    // TODO test
    pub fn calculate_season_points(&mut self, race_points:[RacePoints; NUMBER_OF_RACES_IN_A_SEASON]) {
        let mut season_points = 0;

        for race_point in race_points {
            season_points += race_point.race_points
        }

        self.season_points = season_points
    }
}

#[cfg(test)]
mod season_points_should {
    use super::*;
    use rstest::rstest;

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
