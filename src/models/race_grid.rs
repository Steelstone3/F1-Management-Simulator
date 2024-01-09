use super::teams::team::Team;

pub struct RaceGrid {
    teams: [Team; 10],
}

impl RaceGrid {
    pub fn new() -> Self {
        Self {
            teams: [
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
                Team::new(),
            ],
        }
    }
}

#[cfg(test)]
mod grid_should {}
