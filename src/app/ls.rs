use crate::{sys::term::Term, fs::FS};

pub fn run(fs: &FS, path: String) {
    Term::write(path.as_str());
    let results = fs.ls(path);

    for file in results {
        let out = format!("\n{}", file.name);
        Term::write(out.as_str());
    } 
}
