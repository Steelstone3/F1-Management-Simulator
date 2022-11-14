use std::fmt::{Display, Formatter};

pub enum DriverName {
    LewisHamilton,
    GeorgeRussell,
    MaxVerstappen,
    SergioPerez,
    CarlosSainz,
    CharlesLeclerc,
}

impl Display for DriverName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverName::LewisHamilton => {
                write!(formatter, "Lewis Hamilton",)
            }

            DriverName::GeorgeRussell => {
                write!(formatter, "George Russell",)
            }

            DriverName::MaxVerstappen => {
                write!(formatter, "Max Verstappen",)
            }

            DriverName::SergioPerez => {
                write!(formatter, "Sergio Perez",)
            }

            DriverName::CarlosSainz => {
                write!(formatter, "Carlos Sainz",)
            }

            DriverName::CharlesLeclerc => {
                write!(formatter, "Charles Leclerc",)
            }
        }
    }
}

#[cfg(test)]
mod driver_name_should {
    use crate::models::driver_name::DriverName;
    use rstest::rstest;

    #[rstest]
    #[case("Lewis Hamilton", DriverName::LewisHamilton.to_string())]
    #[case("George Russell", DriverName::GeorgeRussell.to_string())]
    #[case("Max Verstappen", DriverName::MaxVerstappen.to_string())]
    #[case("Sergio Perez", DriverName::SergioPerez.to_string())]
    #[case("Carlos Sainz", DriverName::CarlosSainz.to_string())]
    #[case("Charles Leclerc", DriverName::CharlesLeclerc.to_string())]
    fn display_a_driver_name(#[case] expected_name: String, #[case] actual_name: String) {
        assert_eq!(expected_name, actual_name);
    }
}
