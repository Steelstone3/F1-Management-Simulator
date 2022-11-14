use super::driver_name::DriverName;

#[derive(Debug, PartialEq)]
pub struct Driver {
    pub driver_name: DriverName,
    pub expierence: u8,
    pub race_craft: u8,
    pub awareness: u8,
    pub pace: u8
}