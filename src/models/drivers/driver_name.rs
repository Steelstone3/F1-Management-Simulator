use rand_derive2::RandGen;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(RandGen, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DriverName {
    MaxVerstappen,
    SergioPerez,
    CharlesLeclerc,
    CarlosSainz,
    #[default]
    LewisHamilton,
    GeorgeRussell,
    FernandoAlonso,
    EstebanOcon,
    LandoNorris,
    DanielRicciardo,
    ValtteriBottas,
    GuanyuZhou,
    SebastianVettel,
    LanceStroll,
    KevinMagnussen,
    MickSchumacher,
    PierreGasly,
    YukiTsunoda,
    AlexanderAlbon,
    NicholasLatifi,
}

impl Display for DriverName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverName::MaxVerstappen => {
                write!(formatter, "Max Verstappen",)
            }
            DriverName::SergioPerez => {
                write!(formatter, "Sergio Perez",)
            }
            DriverName::CharlesLeclerc => {
                write!(formatter, "Charles Leclerc",)
            }
            DriverName::CarlosSainz => {
                write!(formatter, "Carlos Sainz",)
            }
            DriverName::LewisHamilton => {
                write!(formatter, "Lewis Hamilton",)
            }
            DriverName::GeorgeRussell => {
                write!(formatter, "George Russell",)
            }
            DriverName::FernandoAlonso => {
                write!(formatter, "Fernando Alonso",)
            }
            DriverName::EstebanOcon => {
                write!(formatter, "Esteban Ocon",)
            }
            DriverName::LandoNorris => {
                write!(formatter, "Lando Norris",)
            }
            DriverName::DanielRicciardo => {
                write!(formatter, "Daniel Ricciardo",)
            }
            DriverName::ValtteriBottas => {
                write!(formatter, "Valtteri Bottas",)
            }
            DriverName::GuanyuZhou => {
                write!(formatter, "Guanyu Zhou",)
            }
            DriverName::SebastianVettel => {
                write!(formatter, "Sebastian Vettel",)
            }
            DriverName::LanceStroll => {
                write!(formatter, "Lance Stroll",)
            }
            DriverName::KevinMagnussen => {
                write!(formatter, "Kevin Magnussen",)
            }
            DriverName::MickSchumacher => {
                write!(formatter, "Mick Schumacher",)
            }
            DriverName::PierreGasly => {
                write!(formatter, "Pierre Gasly",)
            }
            DriverName::YukiTsunoda => {
                write!(formatter, "Yuki Tsunoda",)
            }
            DriverName::AlexanderAlbon => {
                write!(formatter, "Alexander Albon",)
            }
            DriverName::NicholasLatifi => {
                write!(formatter, "Nicholas Latifi",)
            }
        }
    }
}

#[cfg(test)]
mod driver_name_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Max Verstappen", DriverName::MaxVerstappen.to_string())]
    #[case("Sergio Perez", DriverName::SergioPerez.to_string())]
    #[case("Charles Leclerc", DriverName::CharlesLeclerc.to_string())]
    #[case("Carlos Sainz", DriverName::CarlosSainz.to_string())]
    #[case("Lewis Hamilton", DriverName::LewisHamilton.to_string())]
    #[case("George Russell", DriverName::GeorgeRussell.to_string())]
    #[case("Fernando Alonso", DriverName::FernandoAlonso.to_string())]
    #[case("Esteban Ocon", DriverName::EstebanOcon.to_string())]
    #[case("Lando Norris", DriverName::LandoNorris.to_string())]
    #[case("Daniel Ricciardo", DriverName::DanielRicciardo.to_string())]
    #[case("Valtteri Bottas", DriverName::ValtteriBottas.to_string())]
    #[case("Guanyu Zhou", DriverName::GuanyuZhou.to_string())]
    #[case("Sebastian Vettel", DriverName::SebastianVettel.to_string())]
    #[case("Lance Stroll", DriverName::LanceStroll.to_string())]
    #[case("Kevin Magnussen", DriverName::KevinMagnussen.to_string())]
    #[case("Mick Schumacher", DriverName::MickSchumacher.to_string())]
    #[case("Pierre Gasly", DriverName::PierreGasly.to_string())]
    #[case("Yuki Tsunoda", DriverName::YukiTsunoda.to_string())]
    #[case("Alexander Albon", DriverName::AlexanderAlbon.to_string())]
    #[case("Nicholas Latifi", DriverName::NicholasLatifi.to_string())]
    fn display_a_driver_name(#[case] expected_name: String, #[case] actual_name: String) {
        // Then
        assert_eq!(expected_name, actual_name);
    }
}
