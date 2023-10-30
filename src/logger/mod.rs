use std::fs::OpenOptions;
use std::io::*;

pub struct Logger {}

impl Logger {
   pub fn log(data: String) {
    let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open("log.json")
      .unwrap();

    let _ = writeln!(file, "{}", data);
  }
}