use presenters::presenter::{Presenter, UserInterface};

mod presenters;

fn main() {
    UserInterface::write("hello world".to_string());
}
