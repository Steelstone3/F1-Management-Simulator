use rand::Rng;

pub fn assign_random_range(max: u8) -> u8 {
    let mut rng = rand::thread_rng();

    rng.gen_range(1..max)
}