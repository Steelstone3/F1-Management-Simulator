#[derive(Debug, PartialEq, Eq)]
pub struct Points {
    pub race_points: [u32; 10],
    // Make private
    pub points_system: [u32; 10],
}

impl Points {
    pub fn calculate_season_points(&self) -> u32 {
        let mut season_points = 0;

        for points in self.race_points {
            season_points += points
        }

        season_points
    }

    pub fn calculate_points_for_finish_position(&self, finish_position: usize) -> u32 {
        if finish_position < 1 || finish_position > 10 {
            return 0
        }

        self.points_system[finish_position - 1]
    }
}

impl Default for Points {
    fn default() -> Self {
        Self {
            race_points: Default::default(),
            points_system: [25, 18, 15, 12, 10, 8, 6, 4, 2, 1],
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
        #[case] race_points: [u32; 10],
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
        // Given
        let points = Points::default();

        // When
        let points_score = points.calculate_points_for_finish_position(finish_position);

        // Then
        assert_eq!(expected_points_score, points_score);
    }
}
