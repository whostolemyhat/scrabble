use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::error::Error;

// use serde_json::{from_str, to_string};

type Dictionary = HashMap<String, Vec<String>>;

fn generate_dict () -> Result<(), Box<dyn Error>> {
  let mut map: Dictionary = HashMap::new();

  let file = File::open("dictionary.txt")?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let mut chars: Vec<char> = line.as_str().chars().collect();
    chars.sort();
    let ordered = chars.into_iter().collect::<String>();
    // println!("{:?} {:?}", &line, &ordered);
    map.entry(ordered).and_modify(|m| m.push(line.clone())).or_insert(vec![line.clone()]);
  }

  // let json = to_string(&map)?;
  // println!("{:?}", json);

  let output = File::create("dict.cbor")?;
  // output.write_all(json.as_bytes())?;
  serde_cbor::to_writer(output, &map)?;

  Ok(())
}

fn load_dict() -> Result<Dictionary, Box<dyn Error>> {
  let dict_file = File::open("dict.cbor")?;
  // let dict_file = read_to_string("dict.cbor")?;
  // let json: Dictionary = from_str(&dynamic)?;
  let dict = serde_cbor::from_reader(dict_file)?;
  Ok(dict)
}

fn main() -> Result<(), Box<dyn Error>> {
  // generate_dict()?;
  let dict = load_dict()?;

  // read in word to find
  // to_lowercase
  // sort letters in word
  // print found
  // handle none
  let found = dict.get("no");
  println!("{:?}", found);

  let found = dict.get("aeprst");
  println!("{:?}", found);

  Ok(())
}
