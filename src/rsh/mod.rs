use std::path::Path;
use std::io::{Write, stdin, stdout};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::sys::term::Term;
use crate::app::neofetch::Neofetch;


pub struct RSH {
    dir: String
}

impl RSH {
    pub fn new() -> RSH {
        RSH { dir: String::from("/") }
    }

    pub fn run(&self) {
        Term::clear(Color::Black);

        Neofetch::run();

        loop {
            let mut command = String::new();
            Term::set_color(Color::White, Some(Color::Blue));
            Term::write(format!("\n {}", self.dir).as_str());
            Term::set_color(Color::White, Some(Color::Black));
            Term::write(" => ");
            Term::flush();

            stdin()
                .read_line(&mut command)
                .expect("error: failed to read from stdin");

        }
    }

    fn execute(command: &str) {
        
    }
}
