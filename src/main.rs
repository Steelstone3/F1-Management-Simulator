use controllers::game::{display_the_drivers_season_standings, run_season, setup, display_the_teams_season_standings};
use presenters::presenter::display_the_grid;

mod controllers;
mod models;
mod presenters;

fn main() {
    let driver_grid = setup();
    display_the_grid(driver_grid);
    let season = run_season(driver_grid);
    display_the_drivers_season_standings(season);
    display_the_teams_season_standings(season);
}
