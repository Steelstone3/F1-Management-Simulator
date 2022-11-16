use rand::random;

use super::{driver_factory::create_grid, random_generator::generate_seed};
use crate::{
    models::{driver::Driver, race::Race},
    presenters::presenter::display_the_race_result,
};

pub fn setup() -> [Driver; 22] {
    create_grid()
}

pub fn run_race(driver_grid: [Driver; 22]) -> Race {
    let mut race = Race {
        name: random(),
        race_results: driver_grid,
    };

    race.calculate_race_chances([
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
        generate_seed(),
        generate_seed(),
    ]);
    race.sort_racing_result_order();
    race.assign_points();

    display_the_race_result(race);

    race
}

pub fn run_season(driver_grid: [Driver; 22]) -> [Race; 22] {
    [
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
        run_race(driver_grid),
    ]
}

pub fn display_the_season_standings(races: [Race; 22]) {}
