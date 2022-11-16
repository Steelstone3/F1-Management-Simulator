use crate::{
    models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName},
    presenters::presenter::{select_driver_name, select_team_name},
};

use super::random_generator::generate_seed;

pub fn create_grid() -> [Driver; 22] {
    let player_team_name = select_team_name();
    let player_team = create_team(player_team_name);
    let red_bull = create_team(TeamName::RedBull);
    let ferrai = create_team(TeamName::Ferrari);
    let mercedes = create_team(TeamName::Mercedes);
    let alpine = create_team(TeamName::Alpine);
    let mclaren = create_team(TeamName::Mclaren);
    let alpha_romeo = create_team(TeamName::AlphaRomeo);
    let aston_martin = create_team(TeamName::AstonMartin);
    let haas = create_team(TeamName::Haas);
    let alpha_tauri = create_team(TeamName::AlphaTauri);
    let williams = create_team(TeamName::Williams);

    [
        create_driver(select_driver_name(), player_team),
        create_driver(select_driver_name(), player_team),
        create_driver(DriverName::MaxVerstappen, red_bull),
        create_driver(DriverName::SergioPerez, red_bull),
        create_driver(DriverName::CharlesLeclerc, ferrai),
        create_driver(DriverName::CarlosSainz, ferrai),
        create_driver(DriverName::LewisHamilton, mercedes),
        create_driver(DriverName::GeorgeRussell, mercedes),
        create_driver(DriverName::FernandoAlonso, alpine),
        create_driver(DriverName::EstebanOcon, alpine),
        create_driver(DriverName::LandoNorris, mclaren),
        create_driver(DriverName::DanielRicciardo, mclaren),
        create_driver(DriverName::ValtteriBottas, alpha_romeo),
        create_driver(DriverName::GuanyuZhou, alpha_romeo),
        create_driver(DriverName::SebastianVettel, aston_martin),
        create_driver(DriverName::LanceStroll, aston_martin),
        create_driver(DriverName::KevinMagnussen, haas),
        create_driver(DriverName::MickSchumacher, haas),
        create_driver(DriverName::PierreGasly, alpha_tauri),
        create_driver(DriverName::YukiTsunoda, alpha_tauri),
        create_driver(DriverName::AlexanderAlbon, williams),
        create_driver(DriverName::NicholasLatifi, williams),
    ]
}

fn create_driver(driver_name: DriverName, team: Team) -> Driver {
    Driver::new(
        driver_name,
        team,
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}

fn create_team(team_name: TeamName) -> Team {
    Team::new(
        team_name,
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}
