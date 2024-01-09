use controller::random_generator::generate_seed;
use models::drivers::driver::Driver;

mod controller;
mod models;

fn main() {
    let _driver = Driver::new([
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
        generate_seed(),
    ]);
}
