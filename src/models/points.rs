use super::season::NUMBER_OF_RACES_IN_A_SEASON;

pub const RACE_POSIITIONS_THAT_ALLOCATE_POINTS: usize = 10;
pub const POINTS_SYSTEM: [u32; RACE_POSIITIONS_THAT_ALLOCATE_POINTS] =
    [25, 18, 15, 12, 10, 8, 6, 4, 2, 1];

#[derive(Debug, PartialEq, Eq)]
pub struct Points {
    pub race_points: [u32; NUMBER_OF_RACES_IN_A_SEASON],
}

impl Points {
    pub fn calculate_season_points(&self) -> u32 {
        let mut season_points = 0;

        for points in self.race_points {
            season_points += points
        }

        season_points
    }

    pub fn calculate_points_for_finish_position(finish_position: usize) -> u32 {
        if !(1..=RACE_POSIITIONS_THAT_ALLOCATE_POINTS).contains(&finish_position) {
            return 0;
        }

        POINTS_SYSTEM[finish_position - 1]
    }
}

impl Default for Points {
    fn default() -> Self {
        Self {
            race_points: Default::default(),
        }
    }
}

#[cfg(test)]
mod points_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0,0,0,0,0,0,0,0,0,0], 0)]
    #[case([10,0,0,0,0,0,0,0,0,0], 10)]
    #[case([0,0,0,0,0,15,0,0,0,0], 15)]
    #[case([10,8,6,8,15,25,18,25,12,8], 135)]
    #[case([25,25,25,25,25,25,25,25,25,25], 250)]
    fn calculate_the_points_achieved_over_the_season(
        #[case] race_points: [u32; RACE_POSIITIONS_THAT_ALLOCATE_POINTS],
        #[case] expected_season_points: u32,
    ) {
        // Given
        let points = Points {
            race_points,
            ..Default::default()
        };

        // When
        let season_points = points.calculate_season_points();

        // Then
        assert_eq!(expected_season_points, season_points);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 25)]
    #[case(2, 18)]
    #[case(3, 15)]
    #[case(4, 12)]
    #[case(5, 10)]
    #[case(6, 8)]
    #[case(7, 6)]
    #[case(8, 4)]
    #[case(9, 2)]
    #[case(10, 1)]
    #[case(11, 0)]
    fn calculate_the_points_for_a_given_finish_position(
        #[case] finish_position: usize,
        #[case] expected_points_score: u32,
    ) {
        // When
        let points_score = Points::calculate_points_for_finish_position(finish_position);

        // Then
        assert_eq!(expected_points_score, points_score);
    }
}
