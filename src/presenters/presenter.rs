use std::io;
use std::string::String;

pub struct UserInterface {}

pub trait Presenter {
    fn write(message: String);
    fn read_string(message: &str) -> String;
    fn read_numeric_u32(message: &str, lower_bound: u32, upper_bound: u32) -> u32;
}

impl Presenter for UserInterface {
    fn write(message: String) {
        println!("{}", message);
    }

    fn read_string(message: &str) -> String {
        let mut input = String::new();

        println!("{}", message);

        match io::stdin().read_line(&mut input) {
            Ok(_) => print!(""),
            Err(_e) => println!("{}", _e),
        };

        input
    }

    fn read_numeric_u32(message: &str, lower_bound: u32, upper_bound: u32) -> u32 {
        let mut result = u32::MAX;

        while result == u32::MAX || result > upper_bound || result < lower_bound {
            let input = Self::read_string(message);

            result = match input.as_str().trim().parse::<u32>() {
                Ok(result) => result,
                Err(_e) => u32::MAX,
            };
        }

        result
    }
}
