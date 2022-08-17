use crate::presenters::presenter::{Presenter, UserInterface};

pub fn start() {
   UserInterface::write("Hello world!");
}

#[cfg(test)]
mod game_should {
    #[test]
    #[ignore = "Not mocking yet"]
    fn start_the_game() {
        
    }
}
