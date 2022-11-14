pub struct Points {
    pub points_allocation: [u8; 10],
}

impl Default for Points {
    fn default() -> Self {
        Self {
            points_allocation: [25, 18, 15, 12, 10, 8, 6, 4, 2, 1],
        }
    }
}

#[cfg(test)]
mod points_should {
    use super::*;

    #[test]
    fn have_a_score_for_the_top_ten() {
        let points_allocation = [25, 18, 15, 12, 10, 8, 6, 4, 2, 1];
        let points = Points::default();

        assert_eq!(points_allocation, points.points_allocation);
    }
}
