use fs::FS;

use crate::rsh::RSH;

mod fs;
mod rsh;
mod sys;
mod app;

fn main() {
    let fs = FS::new();
    let shell: RSH = RSH::new(fs);

    loop {
        shell.run();
    }
}
