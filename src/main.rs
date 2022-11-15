use controllers::game::setup;
use presenters::presenter::display_the_grid;

mod models;
mod presenters;
mod controllers;

fn main() {
    let driver_grid = setup();
    display_the_grid(driver_grid);
}