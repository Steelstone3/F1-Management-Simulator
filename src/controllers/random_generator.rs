use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn get_seeded_random_max_range(seed: u64, max: u8) -> u8 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(1..max)
}

pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(u64::MIN..u64::MAX)
}
