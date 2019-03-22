mod grep;
use std::env;
use grep::grep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut c = Config::new(&args).unwrap();
  c.search();
}