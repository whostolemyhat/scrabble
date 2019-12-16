use std::io::{stdin};
use std::error::Error;
use clap::{App, Arg};
use log::{info};

use scrabble::{find_all};
use scrabble::generate::{Wordlist, generate_dict, load_dict};

// anagrammer
// 1words
// TODO switch json/cbor
// TODO arg to generate
// TODO est score
// TODO gen module
// Todo length > 1
// TODO length < 10?
// TODO sort by length
fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();
  let dict_path = std::env::var("DICT_PATH").unwrap_or(".".to_string());

  let matches = App::new("Scrabble")
    .version("1.0")
    .author("James Baum <@whostolemyhat>")
    .about("Finds allowed words for scrabble-type games")
    .arg(Arg::with_name("wordlist")
      .short("w")
      .long("wordlist")
      .takes_value(true)
      .possible_values(&["sowpods", "twl"])
      .default_value("sowpods")
      .help("Wordlist to use. TWL for US/Thailand, SOWPODS for rest of the world (English-speaking)"))
    .subcommand(
      App::new("generate")
        .about("Parses word list and creates map of anagrams")
        .arg(Arg::with_name("wordlist")
          .short("w")
          .long("wordlist")
          .takes_value(true)
          .possible_values(&["sowpods", "twl"])
          .default_value("sowpods")
          .help("Wordlist to use. TWL for US/Thailand, SOWPODS for rest of the world (English-speaking)"))
    )
    .get_matches();

  if let Some(ref matches) = matches.subcommand_matches("generate") {
    // different list in generate
    let list = match matches.value_of("wordlist").expect("Must provide a word list") {
      "sowpods" => Wordlist::Sowpods,
      "twl" => Wordlist::Twl,
      _ => unreachable![]
    };
    generate_dict(&list, &dict_path)?;
    return Ok(());
  };

  let list = match matches.value_of("wordlist").expect("Must provide a word list") {
    "sowpods" => Wordlist::Sowpods,
    "twl" => Wordlist::Twl,
    _ => unreachable![]
  };

  let dict = load_dict(&list, &dict_path)?;

  // replace with info!
  info!("Loaded dict");
  let mut found: Vec<String>;

  loop {
    let mut input_text = String::new();
    stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      info!("Exiting");
      break;
    }

    found = find_all(&dict, &input_text.trim());

    println!("{:?} {:?}", found, found.len());
  }

  Ok(())
}
