use rand::Rng;

#[allow(dead_code)]
pub fn generate_seed() -> u64 {
    let mut random_number_generator = rand::thread_rng();

    random_number_generator.gen_range(u64::MIN..u64::MAX)
}

#[allow(dead_code)]
pub fn generate_19_seeds() -> [u64; 19] {
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
    ]
}

#[cfg(test)]
mod seeds_should {
    use super::*;

    #[test]
    fn generate_exactly_19_seeds() {
        // When
        let seeds = generate_19_seeds();

        // Then
        assert_eq!(19, seeds.len());
    }
}