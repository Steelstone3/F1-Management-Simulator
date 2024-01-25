use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Default, Clone, Copy)]
pub enum CircuitName {
    #[default]
    Silverstone,
    Donnington,
    RedBullRing,
    SpaFrancorchamps,
    Zandvoort,
    WatkinsGlen,
    MagnyCours,
    PaulRicard,
    Hockenheim,
    Kyalami,
    Jerez,
    Catalunya,
    Portimao,
}

impl Display for CircuitName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitName::Silverstone => write!(formatter, "Silverstone"),
            CircuitName::Donnington => write!(formatter, "Donnington"),
            CircuitName::RedBullRing => write!(formatter, "Redbull Ring"),
            CircuitName::SpaFrancorchamps => write!(formatter, "Spa Francorchamps"),
            CircuitName::Zandvoort => write!(formatter, "Zandvoort"),
            CircuitName::WatkinsGlen => write!(formatter, "Watkins Glen"),
            CircuitName::MagnyCours => write!(formatter, "Magny Cours"),
            CircuitName::PaulRicard => write!(formatter, "Paul Ricard"),
            CircuitName::Hockenheim => write!(formatter, "Hockenheim"),
            CircuitName::Kyalami => write!(formatter, "Kyalami"),
            CircuitName::Jerez => write!(formatter, "Jerez"),
            CircuitName::Catalunya => write!(formatter, "Catalunya"),
            CircuitName::Portimao => write!(formatter, "Portimao"),
        }
    }
}

#[cfg(test)]
mod race_track_name_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Silverstone", CircuitName::Silverstone.to_string())]
    #[case("Donnington", CircuitName::Donnington.to_string())]
    #[case("Redbull Ring", CircuitName::RedBullRing.to_string())]
    #[case("Spa Francorchamps", CircuitName::SpaFrancorchamps.to_string())]
    #[case("Zandvoort", CircuitName::Zandvoort.to_string())]
    #[case("Watkins Glen", CircuitName::WatkinsGlen.to_string())]
    #[case("Magny Cours", CircuitName::MagnyCours.to_string())]
    #[case("Paul Ricard", CircuitName::PaulRicard.to_string())]
    #[case("Hockenheim", CircuitName::Hockenheim.to_string())]
    #[case("Kyalami", CircuitName::Kyalami.to_string())]
    #[case("Jerez", CircuitName::Jerez.to_string())]
    #[case("Catalunya", CircuitName::Catalunya.to_string())]
    #[case("Portimao", CircuitName::Portimao.to_string())]
    fn display_a_race_track_name(#[case] expected_name: String, #[case] actual_name: String) {
        // Then
        assert_eq!(expected_name, actual_name);
    }
}