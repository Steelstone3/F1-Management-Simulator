pub struct Points {
    first: u8,
    second: u8,
    third: u8,
    fourth: u8,
    fifth: u8,
    sixth: u8,
    seventh: u8,
    eighth: u8,
    ninth: u8,
    tenth: u8,
}

impl Default for Points {
    fn default() -> Self {
        Self {
            first: 25,
            second: 18,
            third: 15,
            fourth: 12,
            fifth: 10,
            sixth: 8,
            seventh: 6,
            eighth: 4,
            ninth: 2,
            tenth: 1,
        }
    }
}

#[cfg(test)]
mod points_should {
    use super::*;

    #[test]
    fn have_a_score_for_the_top_ten() {
        let points = Points::default();

        assert_eq!(25, points.first);
        assert_eq!(18, points.second);
        assert_eq!(15, points.third);
        assert_eq!(12, points.fourth);
        assert_eq!(10, points.fifth);
        assert_eq!(8, points.sixth);
        assert_eq!(6, points.seventh);
        assert_eq!(4, points.eighth);
        assert_eq!(2, points.ninth);
        assert_eq!(1, points.tenth);
    }
}
