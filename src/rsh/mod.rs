use std::io::stdin;
use termcolor::Color;
use crate::sys::term::Term;
use crate::app;


pub struct RSH {
    dir: String
}

impl RSH {
    pub fn new() -> RSH {
        RSH { dir: String::from("/") }
    }

    pub fn run(&self) {
        Term::clear(Color::Black);
        app::execute("neofetch", None);


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

            let parts: Vec<&str> = command.split(" ").collect();
            let mut app = String::from(parts[0]);
            app.pop();
            app::execute(&app.as_str(), Some(parts[1..].to_vec()));

        }
    }
}
