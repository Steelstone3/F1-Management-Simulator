use super::{car::Car, team_name::TeamName};
use rand_derive2::RandGen;

#[derive(RandGen, Clone, Copy, Debug, PartialEq)]
pub struct Team {
    pub name: TeamName,
    pub car: Car,
    pub points: u16,
}

impl Team {
    fn new(team_name: TeamName, car: Car) -> Self {
        Self {
            name: team_name,
            car,
            points: Default::default(),
        }
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
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
            points: Default::default(),
        };

        let team = Team::new(
            TeamName::Haas,
            Car {
                aero: 40,
                engine: 78,
                reliability: 67,
                tire_management: 34,
                overall: 76,
            },
        );

        assert_eq!(expected_team, team);
    }
}
