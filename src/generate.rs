use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::path::Path;
use std::error::Error;

pub type Dictionary = HashMap<String, Vec<String>>;

pub fn sort_letters(word: &str) -> String {
  let mut chars: Vec<char> = word.chars().collect();
  chars.sort();
  let ordered = chars.into_iter().collect::<String>();
  ordered
}

pub fn generate_dict(source: &Path) -> Result<(), Box<dyn Error>> {
  let mut map: Dictionary = HashMap::new();

  let file = File::open(source)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let ordered = sort_letters(line.as_str());

    map.entry(ordered).and_modify(|m| m.push(line.clone())).or_insert(vec![line.clone()]);
  }

  let output = File::create("dict.cbor")?;
  serde_cbor::to_writer(output, &map)?;

  Ok(())
}

pub fn generate_json_dict(source: &Path) -> Result<(), Box<dyn Error>> {
  let mut map: Dictionary = HashMap::new();

  let file = File::open(source)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let ordered = sort_letters(line.as_str());

    map.entry(ordered).and_modify(|m| m.push(line.clone())).or_insert(vec![line.clone()]);
  }

  let json = serde_json::to_string(&map)?;

  let mut output = File::create("dict.json")?;
  output.write_all(json.as_bytes())?;

  Ok(())
}

pub fn load_dict() -> Result<Dictionary, Box<dyn Error>> {
  let dict_file = File::open("dict.cbor")?;
  let dict = serde_cbor::from_reader(dict_file)?;
  Ok(dict)
}

pub fn load_json_dict() -> std::io::Result<Dictionary> {
  let dict_file = File::open("dict.json")?;
  let dict = serde_json::from_reader(dict_file)?;
  Ok(dict)
}