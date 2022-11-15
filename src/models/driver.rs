use super::{driver_name::DriverName, team::Team};
use crate::controllers::random_generator::get_seeded_random_max_range;
use rand::{random, rngs::StdRng, Rng, SeedableRng};
use rand_derive2::RandGen;

const MAX_EXPERIENCE: u8 = 25;
const MAX_RANGE: u8 = 99;

#[derive(Clone, Copy, RandGen, Debug, PartialEq)]
pub struct Driver {
    pub name: DriverName,
    pub team: Team,
    pub experience: u8,
    pub race_craft: u8,
    pub awareness: u8,
    pub pace: u8,
    pub overall: u32,
    pub race_chance: f32,
    pub points: u16,
}

impl Driver {
    pub fn new_player(name: DriverName, team: Team, seeds: [u64; 4]) -> Self {
        let mut driver = Self {
            name,
            team,
            experience: get_seeded_random_max_range(seeds[0], MAX_EXPERIENCE),
            race_craft: get_seeded_random_max_range(seeds[1], MAX_RANGE),
            awareness: get_seeded_random_max_range(seeds[2], MAX_RANGE),
            pace: get_seeded_random_max_range(seeds[3], MAX_RANGE),
            overall: Default::default(),
            race_chance: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        driver
    }

    pub fn new_ai(seeds: [u64; 4]) -> Self {
        let mut driver = Self {
            name: random(),
            team: random(),
            experience: get_seeded_random_max_range(seeds[0], MAX_EXPERIENCE),
            race_craft: get_seeded_random_max_range(seeds[1], MAX_RANGE),
            awareness: get_seeded_random_max_range(seeds[2], MAX_RANGE),
            pace: get_seeded_random_max_range(seeds[3], MAX_RANGE),
            overall: Default::default(),
            race_chance: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        driver
    }

    pub fn calculate_race_chance(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let race_factor = rng.gen_range(0.8..1.2);

        self.race_chance = (self.overall + self.team.car.overall) as f32 * race_factor;
    }

    pub fn add_points(&mut self, points: u16) {
        self.points += points;
        self.team.points += points;
    }

    fn calculate_overall(&mut self) {
        // calculate the average of the stats
        self.overall = ((self.experience as u32 * 6)
            + self.race_craft as u32
            + self.awareness as u32
            + self.pace as u32)
            / 4;
    }
}

#[cfg(test)]
mod driver_should {
    use super::*;
    use crate::models::{car::Car, team_name::TeamName};
    use rstest::rstest;

    #[test]
    fn create_a_player_driver() {
        let team = team_test_fixture();

        let expected_driver = Driver {
            name: DriverName::CharlesLeclerc,
            team,
            experience: 2,
            race_craft: 81,
            awareness: 18,
            pace: 8,
            overall: 29,
            race_chance: Default::default(),
            points: Default::default(),
        };

        let driver = Driver::new_player(
            DriverName::CharlesLeclerc,
            team,
            [2, 1, 5, 2],
        );

        assert_eq!(expected_driver, driver);
    }

    #[test]
    fn create_an_ai_driver() {
        let expected_driver = Driver {
            name: DriverName::LewisHamilton,
            team: team_test_fixture(),
            experience: 20,
            race_craft: 8,
            awareness: 64,
            pace: 69,
            overall: 65,
            race_chance: Default::default(),
            points: Default::default(),
        };

        let driver = Driver::new_ai([1, 2, 3, 4]);

        assert_eq!(expected_driver.experience, driver.experience);
        assert_eq!(expected_driver.race_craft, driver.race_craft);
        assert_eq!(expected_driver.awareness, driver.awareness);
        assert_eq!(expected_driver.pace, driver.pace);
        assert_eq!(expected_driver.overall, driver.overall);
        assert_eq!(expected_driver.race_chance, driver.race_chance);
        assert_eq!(expected_driver.points, driver.points);
    }

    #[rstest]
    #[case(7, 80, 75, 84.21585)]
    #[case(65, 78, 45, 153.01982)]
    #[case(90, 90, 60, 177.34935)]
    fn calculate_race_chances_based_on_driver_and_car(
        #[case] driver_overall: u32,
        #[case] car_overall: u32,
        #[case] seed: u64,
        #[case] race_chances: f32,
    ) {
        let mut driver = Driver {
            name: DriverName::LewisHamilton,
            team: team_test_fixture(),
            experience: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: Default::default(),
            race_chance: Default::default(),
            points: Default::default(),
        };
        driver.overall = driver_overall;
        driver.team.car.overall = car_overall;

        driver.calculate_race_chance(seed);

        assert_eq!(race_chances, driver.race_chance);
    }

    #[test]
    fn add_team_and_driver_points() {
        let expected_points = 2000;
        let points = 1000;
        let mut driver = Driver {
            name: DriverName::LewisHamilton,
            team: team_test_fixture(),
            experience: Default::default(),
            race_craft: Default::default(),
            awareness: Default::default(),
            pace: Default::default(),
            overall: Default::default(),
            race_chance: Default::default(),
            points: Default::default(),
        };

        driver.add_points(points);
        driver.add_points(points);

        assert_eq!(expected_points, driver.points);
        assert_eq!(expected_points, driver.team.points);
    }

    #[rstest]
    #[case(1, 1, 1, 1, 2)]
    #[case(10, 20, 30, 40, 37)]
    #[case(50, 90, 20, 70, 120)]
    fn calculate_overall_stat(
        #[case] experience: u8,
        #[case] race_craft: u8,
        #[case] awareness: u8,
        #[case] pace: u8,
        #[case] overall: u32,
    ) {
        let mut driver = Driver {
            name: DriverName::CarlosSainz,
            team: random(),
            experience,
            race_craft,
            awareness,
            pace,
            overall: Default::default(),
            race_chance: Default::default(),
            points: Default::default(),
        };

        driver.calculate_overall();

        assert_eq!(overall, driver.overall);
    }

    fn team_test_fixture() -> Team {
        Team {
            name: TeamName::Haas,
            car: Car::new([1, 2, 3, 4]),
            points: Default::default(),
        }
    }
}
