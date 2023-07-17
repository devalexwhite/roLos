use std::io::stdin;
use termcolor::Color;
use crate::fs::FS;
use crate::sys::term::Term;
use crate::app;


pub struct RSH {
    dir: String,
    fs: FS,
}

impl RSH {
    pub fn new(fs: FS) -> RSH {
        RSH { dir: String::from("/"), fs }
    }

    pub fn run(&self) {
        Term::clear(Color::Black);
        app::execute("neofetch", None, &self.fs);


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

            let mut parts: Vec<&str> = command.split(" ").collect();
            let len = parts.len() - 1;
            let app = String::from(parts[0]);

            let mut last = parts.last().unwrap().to_string();
            last = last.replace("\n", "");

            parts[len] = last.as_str().clone();


            app::execute(&app.as_str(), Some(parts[1..].to_vec()), &self.fs);

        }
    }
}
