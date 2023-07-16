use std::io::{Write, stdin, stdout};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use clearscreen;

pub struct Term { }

impl Term {
    pub fn write(message: &str) {
        print!("{}", message);
    }

    pub fn clear(color: Color) {
        Term::set_color(color, Some(color));
        clearscreen::clear()
            .expect("error, failed to clear screen");
    }

    pub fn set_color(foreground_color: Color, bg_color: Option<Color>) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if let Some(bg) = bg_color {
            stdout.set_color(ColorSpec::new().set_bg(bg_color).set_fg(Some(foreground_color)));
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(foreground_color)));
        }
    }

    pub fn flush() {
        stdout()
            .flush()
            .expect("warning: failed to flush stdout");
    }
}
