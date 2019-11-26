use regex::Regex;

#[macro_use]
extern crate lazy_static;

pub mod generate;

use generate::{Dictionary, sort_letters};

lazy_static! {
  // static ref ALPHABET: Vec<String> = (b'a'..=b'z').map(|c| (c as char).to_string()).collect();
  static ref ALPHABET: [&'static str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
  static ref RE: Regex = Regex::new(r"(\?)").unwrap();
}


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
  let result = RE.find_iter(&word);
  let count = result.count();

  let mut replaced = vec![];

  // TODO throw error
  if count > 2 {
    println!("Max two wildcards");
    return replaced;
  }

  if count == 0 {
    replaced.push(word.to_owned());
    return replaced;
  }

  for letter in &*ALPHABET {
    let single_replacement = RE.replace(&word, *letter);

    if count > 1 {
      for another_letter in ALPHABET.iter() {
        let mega_replaced: String = RE.replace(&single_replacement, *another_letter).into();
        replaced.push(mega_replaced);
      }
    } else {
      replaced.push(single_replacement.into());
    }
  }

  replaced
}

pub fn find_all(dict: &Dictionary, seed: &str) -> Vec<String> {
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

#[cfg(test)]
mod tests {
  use crate::{combinations, sort_letters, get_words, find_all, replace_wildcards};
  use crate::generate::{load_json_dict};

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
    words.sort();
    let mut expected = vec!["eh", "he", "el", "oe", "ho", "oh", "hoe", "lo", "ole", "helo", "hole", "es", "sh", "ehs", "hes", "she", "els", "les", "sel", "os", "so", "oes", "ose", "hos", "ohs", "soh", "hoes", "hose", "shoe", "los", "sol", "lose", "oles", "sloe", "sole", "hols", "losh", "helos", "holes", "hosel", "sheol", "et", "te", "eth", "het", "the", "elt", "let", "tel", "to", "toe", "hot", "tho", "hote", "lot", "lote", "tole", "holt", "loth", "helot", "hotel", "thole", "st", "est", "set", "tes", "eths", "hest", "hets", "shet", "elts", "lest", "lets", "tels", "sot", "toes", "tose", "host", "hots", "shot", "soth", "tosh", "ethos", "shote", "those", "lost", "lots", "slot", "lotes", "stole", "telos", "toles", "holts", "sloth", "helots", "hostel", "hotels", "tholes"];
    expected.sort();
    assert_eq!(words, expected);

    let mut wild_words = find_all(&dict, &"?otel");
    let mut wild_expected = vec!["ae", "al", "ale", "aloe", "alt", "alto", "at", "ate", "be", "bel", "belt", "bet", "blet", "blot", "bo", "boet", "bole", "bolt", "bot", "botel", "cel", "celt", "clot", "clote", "col", "cole", "colt", "cot", "cote", "de", "del", "delo", "delt", "do", "doe", "dol", "dole", "dolt", "dot", "dote", "ea", "eat", "eco", "ed", "ee", "eel", "ef", "eft", "ego", "eh", "el", "eld", "elf", "elk", "ell", "elm", "els", "elt", "elts", "em", "emo", "en", "enol", "eon", "eorl", "er", "es", "est", "et", "eta", "eth", "evo", "ewt", "ex", "exo", "extol", "eyot", "fe", "felt", "fet", "floe", "flote", "foe", "gel", "gelt", "geo", "get", "go", "goe", "goel", "gole", "got", "he", "helo", "helot", "het", "ho", "hoe", "hole", "holt", "hot", "hote", "hotel", "io", "it", "jet", "jo", "joe", "jol", "jole", "jolt", "jot", "kelt", "ket", "keto", "ketol", "ko", "koel", "la", "lat", "late", "lea", "leat", "led", "lee", "leet", "left", "leg", "lei", "lek", "leno", "lent", "lento", "lep", "lept", "les", "lest", "let", "lets", "leu", "lev", "levo", "lew", "lex", "ley", "lez", "li", "lie", "lit", "lite", "lo", "lob", "lobe", "lod", "lode", "loft", "log", "loge", "loke", "lome", "lone", "loo", "loot", "lop", "lope", "lor", "lore", "los", "lose", "lost", "lot", "lota", "lote", "lotes", "loth", "loti", "loto", "lots", "lotte", "lou", "lout", "love", "low", "lowe", "lowt", "lox", "loy", "lute", "lye", "lyte", "me", "mel", "melt", "met", "metol", "mo", "moe", "mol", "mole", "molt", "mot", "mote", "motel", "ne", "net", "no", "noel", "nole", "not", "note", "oat", "ob", "obe", "od", "ode", "oe", "oes", "of", "oft", "ogle", "oh", "oi", "oil", "oke", "old", "ole", "olea", "olent", "oleo", "oles", "olm", "olpe", "om", "on", "one", "oo", "oot", "op", "ope", "opt", "or", "ore", "orle", "ort", "os", "ose", "ou", "out", "ovel", "ow", "owe", "owl", "owlet", "owt", "ox", "oy", "oye", "pe", "pelt", "pet", "plot", "po", "poet", "pol", "pole", "polt", "pot", "pote", "re", "reo", "ret", "roe", "role", "rot", "rote", "rotl", "sel", "set", "sloe", "slot", "so", "sol", "sole", "sot", "st", "stole", "ta", "tae", "tael", "tale", "tao", "te", "tea", "teal", "tec", "ted", "tee", "teel", "tef", "teg", "teil", "tel", "tela", "telco", "teld", "tele", "tell", "teloi", "telos", "tels", "telt", "ten", "tes", "tet", "tew", "tex", "the", "tho", "thole", "ti", "tie", "til", "tile", "to", "toc", "tod", "toe", "toea", "toed", "toes", "toey", "tog", "toge", "toil", "toile", "toke", "tola", "told", "tole", "toled", "toles", "toll", "tolt", "tolu", "tom", "tome", "ton", "tone", "too", "tool", "top", "tope", "tor", "tore", "tose", "tot", "tote", "tow", "towel", "toy", "toze", "tule", "two", "tye", "ule", "ut", "ute", "vet", "veto", "voe", "vol", "vole", "volet", "volt", "volte", "vote", "we", "welt", "wet", "wo", "woe", "wot", "ye", "yelt", "yet", "yo", "zel", "zlote", "zo", "zol"];
    wild_words.sort();
    wild_expected.sort();

    assert_eq!(wild_words, wild_expected);
  }

  #[test]
  fn test_wildcard() {
    assert_eq!(replace_wildcards("bob"), vec!["bob"]);
    assert_eq!(replace_wildcards("bo?b"), vec!["boab", "bobb", "bocb", "bodb", "boeb", "bofb", "bogb", "bohb", "boib", "bojb", "bokb", "bolb", "bomb", "bonb", "boob", "bopb", "boqb", "borb", "bosb", "botb", "boub", "bovb", "bowb", "boxb", "boyb", "bozb"]);
    assert_eq!(replace_wildcards("?bo?b"), vec!["aboab", "abobb", "abocb", "abodb", "aboeb", "abofb", "abogb", "abohb", "aboib", "abojb", "abokb", "abolb", "abomb", "abonb", "aboob", "abopb", "aboqb", "aborb", "abosb", "abotb", "aboub", "abovb", "abowb", "aboxb", "aboyb", "abozb", "bboab", "bbobb", "bbocb", "bbodb", "bboeb", "bbofb", "bbogb", "bbohb", "bboib", "bbojb", "bbokb", "bbolb", "bbomb", "bbonb", "bboob", "bbopb", "bboqb", "bborb", "bbosb", "bbotb", "bboub", "bbovb", "bbowb", "bboxb", "bboyb", "bbozb", "cboab", "cbobb", "cbocb", "cbodb", "cboeb", "cbofb", "cbogb", "cbohb", "cboib", "cbojb", "cbokb", "cbolb", "cbomb", "cbonb", "cboob", "cbopb", "cboqb", "cborb", "cbosb", "cbotb", "cboub", "cbovb", "cbowb", "cboxb", "cboyb", "cbozb", "dboab", "dbobb", "dbocb", "dbodb", "dboeb", "dbofb", "dbogb", "dbohb", "dboib", "dbojb", "dbokb", "dbolb", "dbomb", "dbonb", "dboob", "dbopb", "dboqb", "dborb", "dbosb", "dbotb", "dboub", "dbovb", "dbowb", "dboxb", "dboyb", "dbozb", "eboab", "ebobb", "ebocb", "ebodb", "eboeb", "ebofb", "ebogb", "ebohb", "eboib", "ebojb", "ebokb", "ebolb", "ebomb", "ebonb", "eboob", "ebopb", "eboqb", "eborb", "ebosb", "ebotb", "eboub", "ebovb", "ebowb", "eboxb", "eboyb", "ebozb", "fboab", "fbobb", "fbocb", "fbodb", "fboeb", "fbofb", "fbogb", "fbohb", "fboib", "fbojb", "fbokb", "fbolb", "fbomb", "fbonb", "fboob", "fbopb", "fboqb", "fborb", "fbosb", "fbotb", "fboub", "fbovb", "fbowb", "fboxb", "fboyb", "fbozb", "gboab", "gbobb", "gbocb", "gbodb", "gboeb", "gbofb", "gbogb", "gbohb", "gboib", "gbojb", "gbokb", "gbolb", "gbomb", "gbonb", "gboob", "gbopb", "gboqb", "gborb", "gbosb", "gbotb", "gboub", "gbovb", "gbowb", "gboxb", "gboyb", "gbozb", "hboab", "hbobb", "hbocb", "hbodb", "hboeb", "hbofb", "hbogb", "hbohb", "hboib", "hbojb", "hbokb", "hbolb", "hbomb", "hbonb", "hboob", "hbopb", "hboqb", "hborb", "hbosb", "hbotb", "hboub", "hbovb", "hbowb", "hboxb", "hboyb", "hbozb", "iboab", "ibobb", "ibocb", "ibodb", "iboeb", "ibofb", "ibogb", "ibohb", "iboib", "ibojb", "ibokb", "ibolb", "ibomb", "ibonb", "iboob", "ibopb", "iboqb", "iborb", "ibosb", "ibotb", "iboub", "ibovb", "ibowb", "iboxb", "iboyb", "ibozb", "jboab", "jbobb", "jbocb", "jbodb", "jboeb", "jbofb", "jbogb", "jbohb", "jboib", "jbojb", "jbokb", "jbolb", "jbomb", "jbonb", "jboob", "jbopb", "jboqb", "jborb", "jbosb", "jbotb", "jboub", "jbovb", "jbowb", "jboxb", "jboyb", "jbozb", "kboab", "kbobb", "kbocb", "kbodb", "kboeb", "kbofb", "kbogb", "kbohb", "kboib", "kbojb", "kbokb", "kbolb", "kbomb", "kbonb", "kboob", "kbopb", "kboqb", "kborb", "kbosb", "kbotb", "kboub", "kbovb", "kbowb", "kboxb", "kboyb", "kbozb", "lboab", "lbobb", "lbocb", "lbodb", "lboeb", "lbofb", "lbogb", "lbohb", "lboib", "lbojb", "lbokb", "lbolb", "lbomb", "lbonb", "lboob", "lbopb", "lboqb", "lborb", "lbosb", "lbotb", "lboub", "lbovb", "lbowb", "lboxb", "lboyb", "lbozb", "mboab", "mbobb", "mbocb", "mbodb", "mboeb", "mbofb", "mbogb", "mbohb", "mboib", "mbojb", "mbokb", "mbolb", "mbomb", "mbonb", "mboob", "mbopb", "mboqb", "mborb", "mbosb", "mbotb", "mboub", "mbovb", "mbowb", "mboxb", "mboyb", "mbozb", "nboab", "nbobb", "nbocb", "nbodb", "nboeb", "nbofb", "nbogb", "nbohb", "nboib", "nbojb", "nbokb", "nbolb", "nbomb", "nbonb", "nboob", "nbopb", "nboqb", "nborb", "nbosb", "nbotb", "nboub", "nbovb", "nbowb", "nboxb", "nboyb", "nbozb", "oboab", "obobb", "obocb", "obodb", "oboeb", "obofb", "obogb", "obohb", "oboib", "obojb", "obokb", "obolb", "obomb", "obonb", "oboob", "obopb", "oboqb", "oborb", "obosb", "obotb", "oboub", "obovb", "obowb", "oboxb", "oboyb", "obozb", "pboab", "pbobb", "pbocb", "pbodb", "pboeb", "pbofb", "pbogb", "pbohb", "pboib", "pbojb", "pbokb", "pbolb", "pbomb", "pbonb", "pboob", "pbopb", "pboqb", "pborb", "pbosb", "pbotb", "pboub", "pbovb", "pbowb", "pboxb", "pboyb", "pbozb", "qboab", "qbobb", "qbocb", "qbodb", "qboeb", "qbofb", "qbogb", "qbohb", "qboib", "qbojb", "qbokb", "qbolb", "qbomb", "qbonb", "qboob", "qbopb", "qboqb", "qborb", "qbosb", "qbotb", "qboub", "qbovb", "qbowb", "qboxb", "qboyb", "qbozb", "rboab", "rbobb", "rbocb", "rbodb", "rboeb", "rbofb", "rbogb", "rbohb", "rboib", "rbojb", "rbokb", "rbolb", "rbomb", "rbonb", "rboob", "rbopb", "rboqb", "rborb", "rbosb", "rbotb", "rboub", "rbovb", "rbowb", "rboxb", "rboyb", "rbozb", "sboab", "sbobb", "sbocb", "sbodb", "sboeb", "sbofb", "sbogb", "sbohb", "sboib", "sbojb", "sbokb", "sbolb", "sbomb", "sbonb", "sboob", "sbopb", "sboqb", "sborb", "sbosb", "sbotb", "sboub", "sbovb", "sbowb", "sboxb", "sboyb", "sbozb", "tboab", "tbobb", "tbocb", "tbodb", "tboeb", "tbofb", "tbogb", "tbohb", "tboib", "tbojb", "tbokb", "tbolb", "tbomb", "tbonb", "tboob", "tbopb", "tboqb", "tborb", "tbosb", "tbotb", "tboub", "tbovb", "tbowb", "tboxb", "tboyb", "tbozb", "uboab", "ubobb", "ubocb", "ubodb", "uboeb", "ubofb", "ubogb", "ubohb", "uboib", "ubojb", "ubokb", "ubolb", "ubomb", "ubonb", "uboob", "ubopb", "uboqb", "uborb", "ubosb", "ubotb", "uboub", "ubovb", "ubowb", "uboxb", "uboyb", "ubozb", "vboab", "vbobb", "vbocb", "vbodb", "vboeb", "vbofb", "vbogb", "vbohb", "vboib", "vbojb", "vbokb", "vbolb", "vbomb", "vbonb", "vboob", "vbopb", "vboqb", "vborb", "vbosb", "vbotb", "vboub", "vbovb", "vbowb", "vboxb", "vboyb", "vbozb", "wboab", "wbobb", "wbocb", "wbodb", "wboeb", "wbofb", "wbogb", "wbohb", "wboib", "wbojb", "wbokb", "wbolb", "wbomb", "wbonb", "wboob", "wbopb", "wboqb", "wborb", "wbosb", "wbotb", "wboub", "wbovb", "wbowb", "wboxb", "wboyb", "wbozb", "xboab", "xbobb", "xbocb", "xbodb", "xboeb", "xbofb", "xbogb", "xbohb", "xboib", "xbojb", "xbokb", "xbolb", "xbomb", "xbonb", "xboob", "xbopb", "xboqb", "xborb", "xbosb", "xbotb", "xboub", "xbovb", "xbowb", "xboxb", "xboyb", "xbozb", "yboab", "ybobb", "ybocb", "ybodb", "yboeb", "ybofb", "ybogb", "ybohb", "yboib", "ybojb", "ybokb", "ybolb", "ybomb", "ybonb", "yboob", "ybopb", "yboqb", "yborb", "ybosb", "ybotb", "yboub", "ybovb", "ybowb", "yboxb", "yboyb", "ybozb", "zboab", "zbobb", "zbocb", "zbodb", "zboeb", "zbofb", "zbogb", "zbohb", "zboib", "zbojb", "zbokb", "zbolb", "zbomb", "zbonb", "zboob", "zbopb", "zboqb", "zborb", "zbosb", "zbotb", "zboub", "zbovb", "zbowb", "zboxb", "zboyb", "zbozb"]);
  }
}
