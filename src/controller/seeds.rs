use rand::Rng;

#[allow(dead_code)]
pub fn generate_seed() -> u64 {
    let mut random_number_generator = rand::thread_rng();

    random_number_generator.gen_range(u64::MIN..u64::MAX)
}

#[allow(dead_code)]
pub fn generate_20_seeds() -> [u64; 20] {
    [
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
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
    fn generate_exactly_20_seeds() {
        // When
        let seeds = generate_20_seeds();

        // Then
        assert_eq!(20, seeds.len());
    }
}