use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;

pub struct Init {
    target_path: PathBuf,
}

impl Init {

    pub fn new(target_path: PathBuf) -> Init {
        Init { target_path: target_path }
    }

    pub fn execute(&self) {
        if Path::new(&self.target_path.join(".aliases")).exists() {
            println!("Directory already initialized.");
        } else {
            let mut template_file = self.template_file();
            let mut template_contents = String::new();
            template_file.read_to_string(&mut template_contents).unwrap();
            let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
            let array = template_contents.as_bytes();
            new_file.write_all(array);
        }
    }

    // ------------ private ---------- //

    fn template_file(&self) -> File {
        let template_file_path = Path::new("src/templates/aliases"); // how will I work out this path?
        File::open(template_file_path).unwrap()
    }
}