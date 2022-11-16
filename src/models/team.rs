use super::{car::Car, team_name::TeamName};
use rand_derive2::RandGen;

#[derive(RandGen, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Team {
    pub name: TeamName,
    pub car: Car,
    pub race_points: u16,
    pub season_points: u16,
}

impl Team {
    pub fn new(team_name: TeamName, seeds: [u64; 4]) -> Self {
        Self {
            name: team_name,
            car: Car::new(seeds),
            race_points: Default::default(),
            season_points: Default::default(),
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Self { name: Default::default(), car: Default::default(), race_points: Default::default(), season_points: Default::default() }
    }
}

#[cfg(test)]
mod team_should {
    use super::*;

    #[test]
    fn create_a_team() {
        let expected_team = Team {
            name: TeamName::Haas,
            car: Car {
                aero: 8,
                engine: 81,
                reliability: 18,
                tire_management: 8,
                overall: 28,
            },
            race_points: Default::default(),
            season_points: Default::default(),
        };

        let team = Team::new(TeamName::Haas, [2, 1, 5, 2]);

        assert_eq!(expected_team, team);
    }
}
