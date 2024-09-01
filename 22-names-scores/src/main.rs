use std::{fs, path::Path};
use names_scores::*;

fn strip_quotes(s: &str) -> &str {
    s.strip_prefix('"').unwrap().strip_suffix('"').unwrap()
}

fn score(pair: (usize, &&str)) -> usize {
    let (i, name) = pair;
    alphabetical_sum(name) * (i + 1)
}

fn main() {
    let file = std::env::args().nth(1).expect("no names file given").to_string();
    let file = Path::new(&file);

    let names_csv: String = fs::read_to_string(file).expect("file does not exist");
    let mut names: Vec<&str> = names_csv.split(',').map(strip_quotes).collect();
    names.sort();
    let sum: usize = names.iter().enumerate().map(score).sum();
    println!("The sum of scores in {file:?} is {sum}");
}
