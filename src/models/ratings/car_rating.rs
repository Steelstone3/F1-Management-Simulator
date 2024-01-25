#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CarRating {
    pub aero: u32,
    pub engine: u32,
    pub reliability: u32,
    pub tire_management: u32,
    pub overall: u32,
}
impl CarRating {
    pub fn new(seeds: [u64; 5]) -> CarRating {
        todo!()
    }
}
