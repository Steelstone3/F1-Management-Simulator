use crate::models::seasons::season_points::RACE_POSIITIONS_THAT_ALLOCATE_POINTS;

pub const POINTS_SYSTEM: [u32; RACE_POSIITIONS_THAT_ALLOCATE_POINTS] =
    [25, 18, 15, 12, 10, 8, 6, 4, 2, 1];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RacePoints {
    pub race_number: usize,
    pub race_points: u32,
}

impl RacePoints {
    pub fn calculate_points_for_finish_position(finish_position: usize) -> u32 {
        if !(1..=RACE_POSIITIONS_THAT_ALLOCATE_POINTS).contains(&finish_position) {
            return 0;
        }

        POINTS_SYSTEM[finish_position - 1]
    }
}

#[cfg(test)]
mod race_points_should {
    use super::*;
    use rstest::rstest;

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
        let points_score = RacePoints::calculate_points_for_finish_position(finish_position);

        // Then
        assert_eq!(expected_points_score, points_score);
    }
}
