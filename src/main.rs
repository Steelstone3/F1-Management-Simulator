use controllers::game::{run_season, setup};
use presenters::presenter::display_the_grid;

mod controllers;
mod models;
mod presenters;

fn main() {
    let driver_grid = setup();
    display_the_grid(driver_grid);
    run_season(driver_grid);
}
