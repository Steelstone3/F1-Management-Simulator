use rand::Rng;

#[allow(dead_code)]
pub fn generate_seed() -> u64 {
    let mut random_number_generator = rand::thread_rng();

    random_number_generator.gen_range(u64::MIN..u64::MAX)
}

#[allow(dead_code)]
pub fn generate_4_seeds() -> [u64; 4] {
    [
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
    ]
}

#[allow(dead_code)]
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
mod seeds_should {
    use super::*;

    #[test]
    fn generate_exactly_4_seeds() {
        // When
        let seeds = generate_4_seeds();

        // Then
        assert_eq!(4, seeds.len());
    }

    #[test]
    fn generate_exactly_4_seeds() {
        // When
        let seeds = generate_5_seeds();

        // Then
        assert_eq!(5, seeds.len());
    }
}