use super::teams::team::Team;

pub struct Grid {
    teams: [Team; 10],
}

impl Grid {
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
