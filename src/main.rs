use controller::random_generator::generate_seed;
use models::teams::team::Team;

mod controller;
mod models;

fn main() {
    let _team = Team::new(
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    );
}
