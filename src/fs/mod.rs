use vfs::{VfsPath, PhysicalFS };

pub struct File {
    pub name: String,
    pub path: String,
    pub len: u64,
}

pub struct FS {
    root: VfsPath,
}

impl FS {
    pub fn new() -> FS {
        let root = PhysicalFS::new("/home/alex/").into();

        FS { root }
    }

    pub fn ls(&self, path: String) -> Vec<File> {
        let mut contents: Vec<File> = Vec::new();

        let target = self.root.join(path).unwrap();

        let is_exist = target.exists().unwrap_or(false);
        let is_dir = target.is_dir().unwrap_or(false);

        if is_exist && is_dir {
            let mut items: Vec<VfsPath> = target.read_dir().unwrap().collect();

            items.sort_by_key(|path| path.as_str().to_string());

            for item in items {
                let name: String = item.filename();
                let path = item.as_str().to_string();
                

                if name.starts_with(".") {
                    continue;
                }

                let mut len: u64 = 0;
                if let Ok(metadata) = item.metadata() {
                    len = metadata.len;
                }

                let file = File {
                    path,
                    name,
                    len,
                };

                contents.push(file);
            }
        }

        contents
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
