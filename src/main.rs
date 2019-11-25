use std::io::{stdin};
use std::error::Error;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod generate;

use generate::{Dictionary, sort_letters};

fn get_words(dict: &Dictionary, input: &String) -> Vec<String> {
  let word = sort_letters(input.to_lowercase().as_str());
  let combos = combinations(&word.as_str());
  let mut words = vec![];

  for c in combos {
    if let Some(found) = dict.get(&c) {
      words.extend(found.clone())
    }
  }

  words
}

// https://www.reddit.com/r/rust/comments/91h6t8/generating_all_possible_case_variations_of_a/
// TODO fix to ascii?
fn combinations(word: &str) -> Vec<String> {
  let len = word.chars().count();
  let mut cases: Vec<String> = Vec::new();

  for i in 0..u64::pow(2, len as u32) {
    let mut s = String::with_capacity(len);
    for (idx, ch) in word.chars().enumerate() {
      if ((i >> idx) & 1) == 1 {
        s.push_str(&ch.to_string())
      }
    }

    cases.push(s);
  }

  // remove anything less than 2 chars (min for scrabble)
  let filtered: Vec<String> = cases.into_iter().filter(|x| x.len() > 1).collect();

  filtered
}

fn replace_wildcards(word: &str) -> Vec<String> {
  // lazy static or just have a prebuilt array
  let alphabet: Vec<String> = (b'a'..=b'z').map(|c| (c as char).to_string()).collect();

  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\?)").unwrap();
  }
  // println!("regex, {:?}", RE.is_match(&word));
  let result = RE.find_iter(&word);
  // println!("res {:?}", &result.count());
  let count = result.count();

  let mut replaced = vec![];
  if count == 0 {
    replaced.push(word.to_owned());
    return replaced;
  }

  for letter in &alphabet {;
    let single_replacement = RE.replace(&word, letter.as_str());

    if count > 1 {
      for another_letter in &alphabet {
        let mega_replaced: String = RE.replace(&single_replacement, another_letter.as_str()).into();
        replaced.push(mega_replaced);
      }
    } else {
      replaced.push(single_replacement.into());
    }
  }

  replaced
}

fn find_all(dict: &Dictionary, seed: &str) -> Vec<String> {
  let expanded = replace_wildcards(seed);
  let mut found: Vec<String> = vec![];
  for word_input in expanded {
    let mut words = get_words(&dict, &word_input);
    found.append(&mut words);
  }

  found.sort();
  found.dedup();

  found
}

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
  let dict = generate::load_dict()?;

  println!("loaded");

  loop {
    let mut input_text = String::new();
    stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      break;
    }

    let found = find_all(&dict, &input_text.trim());

    println!("{:?} {:?}", found, found.len());
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use crate::{combinations, sort_letters, get_words, find_all};
  use crate::generate::{load_json_dict, Dictionary};

  #[test]
  fn test_combinations() {
    assert_eq!(combinations(&"abc"), vec!["ab", "ac", "bc", "abc"]);
    assert_eq!(combinations(&sort_letters("hotels")), vec!["eh", "el", "hl", "ehl", "eo", "ho", "eho", "lo", "elo", "hlo", "ehlo", "es", "hs", "ehs", "ls", "els", "hls", "ehls", "os", "eos", "hos", "ehos", "los", "elos", "hlos", "ehlos", "et", "ht", "eht", "lt", "elt", "hlt", "ehlt", "ot", "eot", "hot", "ehot", "lot", "elot", "hlot", "ehlot", "st", "est", "hst", "ehst", "lst", "elst", "hlst", "ehlst", "ost", "eost", "host", "ehost", "lost", "elost", "hlost", "ehlost"]);
    assert_eq!(combinations(&"hotels"), vec!["ho", "ht", "ot", "hot", "he", "oe", "hoe", "te", "hte", "ote", "hote", "hl", "ol", "hol", "tl", "htl", "otl", "hotl", "el", "hel", "oel", "hoel", "tel", "htel", "otel", "hotel", "hs", "os", "hos", "ts", "hts", "ots", "hots", "es", "hes", "oes", "hoes", "tes", "htes", "otes", "hotes", "ls", "hls", "ols", "hols", "tls", "htls", "otls", "hotls", "els", "hels", "oels", "hoels", "tels", "htels", "otels", "hotels"]);
  }

  #[test]
  fn test_sorting() {
    assert_eq!(sort_letters(&"rats"), "arst");
    assert_eq!(sort_letters(&"hotels"), "ehlost");
    assert_eq!(sort_letters(&"qowfnewonorafnnewnfnewonffnewnfew"), "aeeeeeffffffnnnnnnnnnooooqrwwwwww");
  }

  #[test]
  fn test_moves() {
    let dict = load_json_dict().unwrap();
    let words = get_words(&dict, &"hotels".to_owned());
    assert_eq!(words, vec!["eh", "he", "el", "oe", "ho", "oh", "hoe", "lo", "ole", "helo", "hole", "es", "sh", "ehs", "hes", "she", "els", "les", "sel", "os", "so", "oes", "ose", "hos", "ohs", "soh", "hoes", "hose", "shoe", "los", "sol", "lose", "oles", "sloe", "sole", "hols", "losh", "helos", "holes", "hosel", "sheol", "et", "te", "eth", "het", "the", "elt", "let", "tel", "to", "toe", "hot", "tho", "hote", "lot", "lote", "tole", "holt", "loth", "helot", "hotel", "thole", "st", "est", "set", "tes", "eths", "hest", "hets", "shet", "elts", "lest", "lets", "tels", "sot", "toes", "tose", "host", "hots", "shot", "soth", "tosh", "ethos", "shote", "those", "lost", "lots", "slot", "lotes", "stole", "telos", "toles", "holts", "sloth", "helots", "hostel", "hotels", "tholes"]);
  }

  #[test]
  fn test_find() {
    let dict = load_json_dict().unwrap();
    let mut words = find_all(&dict, &"hotels");
    let mut expected = vec!["eh", "he", "el", "oe", "ho", "oh", "hoe", "lo", "ole", "helo", "hole", "es", "sh", "ehs", "hes", "she", "els", "les", "sel", "os", "so", "oes", "ose", "hos", "ohs", "soh", "hoes", "hose", "shoe", "los", "sol", "lose", "oles", "sloe", "sole", "hols", "losh", "helos", "holes", "hosel", "sheol", "et", "te", "eth", "het", "the", "elt", "let", "tel", "to", "toe", "hot", "tho", "hote", "lot", "lote", "tole", "holt", "loth", "helot", "hotel", "thole", "st", "est", "set", "tes", "eths", "hest", "hets", "shet", "elts", "lest", "lets", "tels", "sot", "toes", "tose", "host", "hots", "shot", "soth", "tosh", "ethos", "shote", "those", "lost", "lots", "slot", "lotes", "stole", "telos", "toles", "holts", "sloth", "helots", "hostel", "hotels", "tholes"];
    assert_eq!(words.sort(), expected.sort());
  }
}
