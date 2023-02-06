use levenshtein_automata::{LevenshteinAutomatonBuilder};
use std::fs::File;
use std::io::{ self, BufRead, Write };
use serde_json;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage:\n\tcargo run FILE_NAME THRESHOLD ALPHABETH\n")
    }
    let file_name = args[1].as_str();
    let alpha = args[3].as_str();
    let t: u8 = args[2].parse().unwrap();

    let lev_automaton_builder = LevenshteinAutomatonBuilder::new(t, false);


    let file = File::open(file_name).unwrap();
    let mut dfas = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let s = line.unwrap();
        let dfa = lev_automaton_builder.build_dfa(s.as_str());
        let dfa_json = dfa.to_json(t as u32, alpha, s.as_str());
        dfas.push(dfa_json);
    }

    let dfas_json = serde_json::to_string(&dfas).unwrap();

    let mut out_file_name = file_name.to_owned();
    out_file_name.push_str(".json");
    let mut out_file = File::create(out_file_name.as_str()).unwrap();
    let res = out_file.write_all(dfas_json.as_bytes());
    res.unwrap();

}
