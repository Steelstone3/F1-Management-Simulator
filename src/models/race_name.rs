use std::fmt::{Display, Formatter};
use rand_derive2::RandGen;

#[derive(RandGen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaceName {
    Monza,
    Silverstone,
    Imola,
    Spa,
    Donnington,
    PaulRicard,
    HockenheimRing,
    Nürburgring,
}

impl Display for RaceName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RaceName::Monza => {
                write!(formatter, "Monza - Italian Grand Prix",)
            }
            RaceName::Silverstone => {
                write!(formatter, "Silverstone - British Grand Prix",)
            }
            RaceName::Imola => {
                write!(formatter, "Imola - Italian Grand Prix",)
            }
            RaceName::Spa => {
                write!(formatter, "Spa Francorchamps - Belgium Grand Prix",)
            }
            RaceName::Donnington => {
                write!(formatter, "Donnington Park - British Grand Prix",)
            }
            RaceName::PaulRicard => {
                write!(formatter, "Paul Ricard - French Grand Prix",)
            }
            RaceName::HockenheimRing => {
                write!(formatter, "Hockenheim Ring - German Grand Prix",)
            }
            RaceName::Nürburgring =>  {
                write!(formatter, "Nürburgring - German Grand Prix",)
            }
        }
    }
}

#[cfg(test)]
mod race_name_should {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("Monza - Italian Grand Prix", RaceName::Monza.to_string())]
    #[case("Imola - Italian Grand Prix", RaceName::Imola.to_string())]
    #[case("Silverstone - British Grand Prix", RaceName::Silverstone.to_string())]
    #[case("Donnington Park - British Grand Prix", RaceName::Donnington.to_string())]
    #[case("Spa Francorchamps - Belgium Grand Prix", RaceName::Spa.to_string())]
    #[case("Paul Ricard - French Grand Prix", RaceName::PaulRicard.to_string())]
    #[case("Nürburgring - German Grand Prix", RaceName::Nürburgring.to_string())]
    #[case("Hockenheim Ring - German Grand Prix", RaceName::HockenheimRing.to_string())]
    fn display_a_driver_name(#[case] expected_name: String, #[case] actual_name: String) {
        assert_eq!(expected_name, actual_name);
    }
}
