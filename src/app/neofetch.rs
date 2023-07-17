use crate::sys::term::Term;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const VERSION: f32 = 0.1;

pub fn run() {
    Term::set_color(Color::Yellow, Some(Color::Black));
    Term::write(r"
                   _              
                  | |              
         _ __ ___ | |     ___  ___ 
        | '__/ _ \| |    / _ \/ __|
        | | | (_) | |___| (_) \__ \
        |_|  \___/\_____/\___/|___/
    ");

    Term::set_color(Color::Blue, Some(Color::Black));

    Term::write("\n");
    Term::write(format!("\t version {}", VERSION).as_str());
    Term::write("\t http://AlexTheUxGuy.com");
    Term::write("\t Alex White, 2023");
    Term::write("\n");
}
