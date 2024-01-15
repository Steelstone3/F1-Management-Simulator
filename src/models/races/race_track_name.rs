use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Default, Clone, Copy)]
pub enum RaceTrackName {
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

impl Display for RaceTrackName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaceTrackName::Silverstone => write!(formatter, "Silverstone"),
            RaceTrackName::Donnington => write!(formatter, "Donnington"),
            RaceTrackName::RedBullRing => write!(formatter, "Redbull Ring"),
            RaceTrackName::SpaFrancorchamps => write!(formatter, "Spa Francorchamps"),
            RaceTrackName::Zandvoort => write!(formatter, "Zandvoort"),
            RaceTrackName::WatkinsGlen => write!(formatter, "Watkins Glen"),
            RaceTrackName::MagnyCours => write!(formatter, "Magny Cours"),
            RaceTrackName::PaulRicard => write!(formatter, "Paul Ricard"),
            RaceTrackName::Hockenheim => write!(formatter, "Hockenheim"),
            RaceTrackName::Kyalami => write!(formatter, "Kyalami"),
            RaceTrackName::Jerez => write!(formatter, "Jerez"),
            RaceTrackName::Catalunya => write!(formatter, "Catalunya"),
            RaceTrackName::Portimao => write!(formatter, "Portimao"),
        }
    }
}

#[cfg(test)]
mod race_track_name_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Silverstone", RaceTrackName::Silverstone.to_string())]
    #[case("Donnington", RaceTrackName::Donnington.to_string())]
    #[case("Redbull Ring", RaceTrackName::RedBullRing.to_string())]
    #[case("Spa Francorchamps", RaceTrackName::SpaFrancorchamps.to_string())]
    #[case("Zandvoort", RaceTrackName::Zandvoort.to_string())]
    #[case("Watkins Glen", RaceTrackName::WatkinsGlen.to_string())]
    #[case("Magny Cours", RaceTrackName::MagnyCours.to_string())]
    #[case("Paul Ricard", RaceTrackName::PaulRicard.to_string())]
    #[case("Hockenheim", RaceTrackName::Hockenheim.to_string())]
    #[case("Kyalami", RaceTrackName::Kyalami.to_string())]
    #[case("Jerez", RaceTrackName::Jerez.to_string())]
    #[case("Catalunya", RaceTrackName::Catalunya.to_string())]
    #[case("Portimao", RaceTrackName::Portimao.to_string())]
    fn display_a_race_track_name(#[case] expected_name: String, #[case] actual_name: String) {
        // Then
        assert_eq!(expected_name, actual_name);
    }
}
