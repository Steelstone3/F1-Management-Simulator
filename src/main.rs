use models::season::Season;

mod controller;
mod models;

fn main() {
    let mut season = Season::default();
    season.calculate_season_results();
}
