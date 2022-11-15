use crate::{
    models::{driver::Driver, driver_name::DriverName, team::Team, team_name::TeamName}, presenters::presenter::{select_driver_name, select_team_name},
};

use super::random_generator::generate_seed;

pub fn create_grid() -> [Driver; 22] {
    [
        create_driver(select_driver_name(), select_team_name()),
        create_driver(select_driver_name(), select_team_name()),
        create_driver(DriverName::MaxVerstappen, TeamName::RedBull),
        create_driver(DriverName::SergioPerez, TeamName::RedBull),
        create_driver(DriverName::CharlesLeclerc, TeamName::Ferrari),
        create_driver(DriverName::CarlosSainz, TeamName::Ferrari),
        create_driver(DriverName::LewisHamilton, TeamName::Mercedes),
        create_driver(DriverName::GeorgeRussell, TeamName::Mercedes),
        create_driver(DriverName::FernandoAlonso, TeamName::Alpine),
        create_driver(DriverName::EstebanOcon, TeamName::Alpine),
        create_driver(DriverName::LandoNorris, TeamName::Mclaren),
        create_driver(DriverName::DanielRicciardo, TeamName::Mclaren),
        create_driver(DriverName::ValtteriBottas, TeamName::AlphaRomeo),
        create_driver(DriverName::GuanyuZhou, TeamName::AlphaRomeo),
        create_driver(DriverName::SebastianVettel, TeamName::AstonMartin),
        create_driver(DriverName::LanceStroll, TeamName::AstonMartin),
        create_driver(DriverName::KevinMagnussen, TeamName::Haas),
        create_driver(DriverName::MickSchumacher, TeamName::Haas),
        create_driver(DriverName::PierreGasly, TeamName::AlphaTauri),
        create_driver(DriverName::YukiTsunoda, TeamName::AlphaTauri),
        create_driver(DriverName::AlexanderAlbon, TeamName::Williams),
        create_driver(DriverName::NicholasLatifi, TeamName::Williams),
    ]
}

fn create_driver(driver_name: DriverName, team_name: TeamName) -> Driver{
    Driver::new_player(
        driver_name,
        Team::new(
            team_name,
            [
                generate_seed(),
                generate_seed(),
                generate_seed(),
                generate_seed(),
            ],
        ),
        [
            generate_seed(),
            generate_seed(),
            generate_seed(),
            generate_seed(),
        ],
    )
}