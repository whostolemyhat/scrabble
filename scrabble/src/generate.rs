use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};

pub type Dictionary = HashMap<String, Vec<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Wordlist {
  Sowpods,
  Twl
}

pub fn sort_letters(word: &str) -> String {
  let mut chars: Vec<char> = word.chars().collect();
  chars.sort();
  let ordered = chars.into_iter().collect::<String>();
  ordered
}

pub fn generate_dict(list: &Wordlist) -> Result<(), Box<dyn Error>> {
  let mut map: Dictionary = HashMap::new();

  let source = match list {
    Wordlist::Sowpods => "sowpods.txt",
    Wordlist::Twl => "twl06.txt"
  };

  let file = File::open(source)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let ordered = sort_letters(line.as_str());

    map.entry(ordered).and_modify(|m| m.push(line.clone())).or_insert(vec![line.clone()]);
  }

  let out_path = format!("{:?}-dict.json", list);
  let mut output = File::create(out_path)?;
  let json = serde_json::to_string(&map)?;
  output.write_all(json.as_bytes())?;

  Ok(())
}

pub fn load_dict(list: &Wordlist) -> Result<Dictionary, Box<dyn Error>> {
  let dict_file = File::open(format!("{:?}-dict.json", list))?;
  let dict = serde_json::from_reader(dict_file)?;
  Ok(dict)
}
