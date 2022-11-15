use rand::random;

use super::{driver_factory::create_grid, random_generator::generate_seed};
use crate::{
    models::{driver::Driver, points::Points, race::Race},
    presenters::presenter::display_the_race_result,
};

pub fn setup() -> [Driver; 22] {
    create_grid()
}

pub fn run_race(driver_grid: [Driver; 22]) {
    let mut race = Race {
        name: random(),
        race_results: driver_grid,
    };

    race.calculate_race_chances(generate_seed());
    race.sort_racing_result_order();
    race.assign_points();

    display_the_race_result(race);
}

pub fn run_season(driver_grid: [Driver; 22]) {
    for _ in 0..22 {
        run_race(driver_grid);
    }
}
