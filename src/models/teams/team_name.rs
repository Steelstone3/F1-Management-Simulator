use rand_derive2::RandGen;
use std::fmt::{Display, Formatter};

#[derive(RandGen, Debug, PartialEq, Eq)]
pub enum TeamName {
    Ferrari,
    Mercedes,
    RedBull,
    Alpine,
    Mclaren,
    Haas,
    AlphaRomeo,
    AlphaTauri,
    AstonMartin,
    Williams,
}

impl Display for TeamName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamName::Ferrari => {
                write!(formatter, "Ferrari",)
            }

            TeamName::Mercedes => {
                write!(formatter, "Mercedes",)
            }

            TeamName::RedBull => {
                write!(formatter, "Red Bull",)
            }

            TeamName::Alpine => {
                write!(formatter, "Alpine",)
            }

            TeamName::Mclaren => {
                write!(formatter, "Mclaren",)
            }

            TeamName::Haas => {
                write!(formatter, "Haas",)
            }

            TeamName::AlphaRomeo => {
                write!(formatter, "Alpha Romeo",)
            }

            TeamName::AlphaTauri => {
                write!(formatter, "Alpha Tauri",)
            }

            TeamName::AstonMartin => {
                write!(formatter, "Aston Martin",)
            }

            TeamName::Williams => {
                write!(formatter, "Williams",)
            }
        }
    }
}

#[cfg(test)]
mod team_name_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Ferrari", TeamName::Ferrari.to_string())]
    #[case("Mercedes", TeamName::Mercedes.to_string())]
    #[case("Red Bull", TeamName::RedBull.to_string())]
    #[case("Alpine", TeamName::Alpine.to_string())]
    #[case("Mclaren", TeamName::Mclaren.to_string())]
    #[case("Haas", TeamName::Haas.to_string())]
    #[case("Alpha Romeo", TeamName::AlphaRomeo.to_string())]
    #[case("Alpha Tauri", TeamName::AlphaTauri.to_string())]
    #[case("Aston Martin", TeamName::AstonMartin.to_string())]
    #[case("Williams", TeamName::Williams.to_string())]
    fn display_a_team_name(#[case] expected_name: String, #[case] actual_name: String) {
        assert_eq!(expected_name, actual_name);
    }
}