use std::io::{stdin};
use std::error::Error;
// use regex::Regex;

// #[macro_use]
// extern crate lazy_static;

use scrabble;
// mod generate;

// use generate::{Dictionary, sort_letters};

// anagrammer
// 1words
// TODO switch json/cbor
// TODO arg to generate
// String vs str vs &str
// TODO est score
// TODO gen module
// Todo length > 1
// TODO length < 10?
// TODO sort by length
fn main() -> Result<(), Box<dyn Error>> {
  // generate_dict()?;
  // let dict = load_dict()?;
  // generate_json_dict(&Path::new("sowpods.txt"))?;
  let dict = scrabble::generate::load_dict()?;

  println!("loaded");

  loop {
    let mut input_text = String::new();
    stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      break;
    }

    let found = scrabble::find_all(&dict, &input_text.trim());

    println!("{:?} {:?}", found, found.len());
  }

  Ok(())
}
