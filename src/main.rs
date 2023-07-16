use crate::fs::FS;
use crate::rsh::RSH;
use std::io::{Write, stdin, stdout};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod fs;
mod rsh;
mod sys;
mod app;


enum Commands {
    pwd,
    dir,
    ls,
}

fn main() {
    let mut file_sys: FS = FS::new();
    let shell: RSH = RSH::new();

    loop {
        shell.run();
        
        // command.pop();

        // let parts: Vec<&str> = command.split(" ").collect();

        // match parts[0] {
            // "ls" => file_sys.ls(),
            // "cd" => file_sys.cd(parts[1]),
            // _ => ()
        // }
    }
}
