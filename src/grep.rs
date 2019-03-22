pub mod grep {
  use std::fs::{File, self, DirEntry};
  use std::io;
  use std::io::prelude::*;
  use std::path::Path;
  use std::str;
  use std::thread;

  pub struct Config {
    pub file_path: String,
    pub query: String,
    pub result: Vec<String>,
  }

  impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
      if args.len() < 3 {
        println!("Please enter 2 arguments");
        return Err("Invalid argument number");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();

      let mut p = Config {file_path: file_path, query: query, result: Vec::new()};

      Ok(p)
    }

    pub fn open_file(name: &String) -> String {
      let mut f = File::open(name).expect("File not found");
      let mut buffer = Vec::new();
      f.read_to_end(&mut buffer).unwrap();
      let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(_e) => "",
      };

      s.to_string()
    }

    fn visit_dirs(query: &String, dir: &Path, cb: &Fn(&String, &DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self::Config::visit_dirs(query, &path, cb)?;
                } else {
                    cb(query, &entry);
                }
            }
        }
        Ok(())
    }

    fn dump_file(query: &String, entry: &DirEntry) {
        let path = entry.path().to_str().unwrap().to_string();
        let q = query.clone().to_string();
        thread::spawn(move || {
            let result = self::Config::open_file(&path);
            for line in result.lines() {
                if line.contains(&q) {
                    println!("{}: {}", path, line);
                }
            }
        });
    }

    pub fn search(&mut self) {
        let from_string = Path::new(&self.file_path);
        let dir = Path::new(&from_string);

        if dir.is_dir() {
            self::Config::visit_dirs(&self.query, &dir, &self::Config::dump_file).unwrap();
        } else {
            let result = self::Config::open_file(&self.file_path);

            for line in result.lines() {

              if line.contains(&self.query) {
                println!("{}: {}", self.file_path, line);
                self.result.push(line.to_string());
              }
            }
        }
    }
  }
}