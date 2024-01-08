use std::fmt::Display;

use crate::controller::random_generator::generate_seeded_random;

#[derive(Debug, PartialEq, Eq)]
pub struct DriverStatistic {
    pub awareness: u32,
    pub consistency: u32,
    pub experience: u32,
    pub race_craft: u32,
    pub pace: u32,
    pub overall: u32,
}

impl DriverStatistic {
    pub fn new(seeds: [u64; 5]) -> Self {
        let awareness = generate_seeded_random(seeds[0]);
        let consistency = generate_seeded_random(seeds[1]);
        let experience = generate_seeded_random(seeds[2]);
        let race_craft = generate_seeded_random(seeds[3]);
        let pace = generate_seeded_random(seeds[4]);

        let overall = (experience + race_craft + awareness + pace + consistency) / 5;

        Self {
            awareness,
            consistency,
            experience,
            pace,
            race_craft,
            overall,
        }
    }
}

impl Display for DriverStatistic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Awareness: {}\nConsistency: {}\nExpierence: {}\nPace: {}\nRace Craft: {}\nOverall: {}",
            self.awareness,
            self.consistency,
            self.experience,
            self.pace,
            self.race_craft,
            self.overall
        )
    }
}

#[cfg(test)]
mod driver_statistics_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1,2,3,4,5], DriverStatistic{ awareness: 90, consistency: 64, experience: 81, race_craft: 84, pace: 66, overall: 77 })]
    #[case([100,200,300,400,500], DriverStatistic{ awareness: 87, consistency: 90, experience: 57, race_craft: 70, pace: 50, overall: 70 })]
    fn new_driver_statistics(#[case] seeds: [u64; 5], #[case] expected: DriverStatistic) {
        // When
        let driver_statistics = DriverStatistic::new(seeds);

        // Then
        assert_eq!(expected, driver_statistics);
    }

    #[test]
    fn display() {
        // Given
        let expected =
            "Awareness: 90\nConsistency: 64\nExpierence: 81\nPace: 66\nRace Craft: 84\nOverall: 77"
                .to_string();

        // When
        let display = DriverStatistic::new([1, 2, 3, 4, 5]).to_string();

        // Then
        assert_eq!(expected, display);
    }
}
