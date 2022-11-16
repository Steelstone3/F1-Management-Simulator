use controllers::game::{display_the_season_standings, run_season, setup};
use presenters::presenter::display_the_grid;

mod controllers;
mod models;
mod presenters;

fn main() {
    let driver_grid = setup();
    display_the_grid(driver_grid);
    let races = run_season(driver_grid);
    display_the_season_standings(races);
}
