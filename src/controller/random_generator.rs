use rand::{rngs::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
pub fn generate_seeded_random(seed: u64) -> u32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(50..99)
}

#[cfg(test)]
mod random_generator_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(100, 87)]
    #[case(54321, 55)]
    #[case(12345, 76)]
    fn generate_seeded_random_returns_expected_value(#[case] seed: u64, #[case] expected: u32) {
        // When
        let actual = generate_seeded_random(seed);

        // Then
        assert_eq!(actual, expected);
    }
}
