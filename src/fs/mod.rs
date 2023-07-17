use std::fs;
use std::path::Path;

pub struct File {
    name: String,
    path: String,
}

pub struct FS { }

impl FS {
    pub fn ls(path: String) -> Vec<File> {
        let os_path = Path::new(path.as_str());

        let mut files: Vec<File> = Vec::new();

        if os_path.is_dir() {
            if let Ok(entries) = fs::read_dir(os_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                       let file = File {
                            name: String::from(entry.file_name().to_str().unwrap()),
                            path: String::from(entry.path().to_str().unwrap())
                        };
                        files.push(file);
                    }
                }
            }
        }
        else {
            println!("error, path is not a directory");
        }
        files
    }

    // pub fn cd(&mut self, path: &str) {
    //     let dir_path = Path::new(&path);

    //     if !dir_path.is_dir() {
    //         println!("error, path is a file");
    //     } else if !dir_path.exists() {
    //         println!("error, path does not exist");
    //     }

    //     self.cur_dir = path.to_string();
    // }


}
