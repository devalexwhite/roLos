use std::str::FromStr;
use strum_macros::EnumString;

use neofetch::Neofetch;

pub mod neofetch;

#[derive(Debug, PartialEq, EnumString)]
pub enum App {
    neofetch,
}

pub fn execute(command: &str, params: Option<Vec<&str>>) {
    if let Ok(app) = App::from_str(command) {
        match app {
            neofetch => Neofetch::run(),
        }
    }
    else {
    }
}
