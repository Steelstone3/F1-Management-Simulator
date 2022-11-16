use rand::random;

use super::{driver_factory::create_grid, random_generator::generate_seed};
use crate::{
    models::{driver::Driver, race::Race, season::Season},
    presenters::presenter::{display_the_race_result, display_the_drivers_season},
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

pub fn run_season(driver_grid: [Driver; 22]) -> Season {
    Season::new([
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
    ])
}

pub fn display_the_drivers_season_standings(mut season: Season) {
    season.reset_season_points();
    season.calculate_driver_season_points();
    let drivers_season_result = season.order_driver_standings();
    display_the_drivers_season(drivers_season_result);
}
