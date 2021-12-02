extern crate clap;
//#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use log;
use simple_logger as sl;
use std::fs;
use std::io::{self, Read};
mod solution;

fn main() -> Result<(), ()> {
  let matches = App::new("Advent of Code 2021 Solutions")
    .arg(Arg::with_name("PART").short("p").index(1))
    .arg(Arg::with_name("INPUT").index(2))
    .arg(Arg::with_name("v").short("v").multiple(true))
    .get_matches();

  let part = matches
    .value_of("PART")
    .unwrap_or_default()
    .parse::<usize>()
    .unwrap_or(1);

  let log_level = match matches.occurrences_of("v") {
    0 => log::Level::Error,
    1 => log::Level::Warn,
    2 => log::Level::Info,
    3 => log::Level::Debug,
    _ => log::Level::Trace,
  };
  if let Err(e) = sl::init_with_level(log_level) {
    eprintln!("sl::init_with_level({}): error {}", log_level, e);
    return Err(());
  }

  let mut buf = String::new();
  if let Some(input) = matches.value_of("INPUT") {
    match fs::File::open(input) {
      Ok(mut file) => {
        if let Err(e) = file.read_to_string(&mut buf) {
          eprintln!("file({})::read_to_string failed:{}", input, e);
          return Err(());
        }
      }
      Err(e) => {
        eprintln!("file({})::open failed:{}", input, e);
        return Err(());
      }
    };
  } else if let Err(e2) = io::stdin().read_to_string(&mut buf) {
    eprintln!("sl::init_with_level({}): error {}", log_level, e2);
    return Err(());
  }

  let result: String;
  if part == 2 {
    result = solution::part2(&buf);
  } else {
    result = solution::part1(&buf);
  }
  println!("{}", result);
  return Ok(());
}
