use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn generate_seeded_random(seed: u64) -> u32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(50..99)
}

pub fn generate_seed() -> u64 {
    let mut random_number_generator = rand::thread_rng();

    random_number_generator.gen_range(u64::MIN..u64::MAX)
}

pub fn generate_4_seeds() -> [u64; 4] {
    [
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
    ]
}

pub fn generate_5_seeds() -> [u64; 5] {
    [
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
    ]
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
