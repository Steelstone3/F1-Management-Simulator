#[derive(Debug, Default, PartialEq, Eq)]
pub struct Points {
    pub race_points: [u32; 10],
}

impl Points {
    pub fn calculate_season_points(&self) -> u32 {
        let mut season_points = 0;

        for points in self.race_points {
            season_points += points
        }

        season_points
    }
}

#[cfg(test)]
mod points_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0,0,0,0,0,0,0,0,0,0], 0)]
    #[case([10,0,0,0,0,0,0,0,0,0], 10)]
    #[case([0,0,0,0,0,10,0,0,0,0], 10)]
    #[case([10,20,30,40,50,60,70,80,90,100], 550)]
    fn calculate_the_points_achieved_over_the_season(#[case] race_points: [u32; 10], #[case] expected_season_points: u32) {
        // Given
        let points = Points { race_points };

        // When
        let season_points = points.calculate_season_points();

        // Then
        assert_eq!(expected_season_points, season_points);
    }
}
