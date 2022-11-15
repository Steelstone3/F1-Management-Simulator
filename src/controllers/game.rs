use super::driver_factory::create_grid;
use crate::models::driver::Driver;

pub fn setup() -> [Driver; 22] {
    create_grid()
}

pub fn run_race() {}
