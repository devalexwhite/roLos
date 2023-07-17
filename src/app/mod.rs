use std::str::FromStr;
use strum_macros::EnumString;

use crate::{fs::FS};

pub mod neofetch;
pub mod ls;

#[derive(Debug, PartialEq, EnumString)]
pub enum App {
    #[strum(serialize="neofetch")]
    Neofetch,
    #[strum(serialize="ls")]
    Ls,
}

pub fn execute(command: &str, params: Option<Vec<&str>>, fs: &FS) {
    if let Ok(app) = App::from_str(command) {
        match app {
            App::Neofetch => neofetch::run(),
            App::Ls => ls::run(fs, params.unwrap()[0].to_string()),
        }
    }
    else {
    }
}
