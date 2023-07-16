use std::fs::{self,DirEntry};
use std::path::Path;

pub struct FS {
    pub cur_dir: String
}

impl FS {
    pub fn new() -> FS {
        FS { cur_dir: String::from("/") }
    }

    pub fn ls(&self) {
        let path = Path::new(&self.cur_dir);

        if (path.is_dir()) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("{}",
                                 entry.path().display(),
                                );
                    }
                }
            }
        }
        else {
            println!("error, path is not a directory");
        }
    }

    pub fn cd(&mut self, path: &str) {
        let dir_path = Path::new(&path);

        if !dir_path.is_dir() {
            println!("error, path is a file");
        } else if !dir_path.exists() {
            println!("error, path does not exist");
        }

        self.cur_dir = path.to_string();
    }


}
