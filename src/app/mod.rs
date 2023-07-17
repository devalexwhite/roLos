use std::str::FromStr;
use strum_macros::EnumString;

use neofetch::Neofetch;
use ls::Ls;

pub mod neofetch;
pub mod ls;

#[derive(Debug, PartialEq, EnumString)]
pub enum App {
    #[strum(serialize="neofetch")]
    Neofetch,
    #[strum(serialize="ls")]
    Ls,
}

pub fn execute(command: &str, params: Option<Vec<&str>>) {
    if let Ok(app) = App::from_str(command) {
        match app {
            App::Neofetch => Neofetch::run(),
            App::Ls => Ls::run(),
        }
    }
    else {
    }
}
