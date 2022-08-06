use std::fs::File;
use std::io::{self};
use regex::Regex;
use itertools::Itertools;

pub fn split_into_words(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    let word_separator = Regex::new(r"([ ,.]+)").expect("Invalid regex");

    let mut words: Vec<String> = vec![];
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let new_words: Vec<String> = word_separator.split(&ip).map(str::to_lowercase).collect();
            words.extend(new_words);
        }
    }
    words   
}

pub fn count_words(words: Vec<String>) -> impl Iterator<Item=(usize, String)> {
    // TODO: Remove unnecessary clones.
    let counts = 
        words.into_iter()
             .filter_map(|s| { if s.trim().is_empty() { None } else { Some((s.clone(), s.clone())) } })
             .into_group_map()
             .into_iter()
             .map (|(key, value)| (value.len(), key))
             .sorted();
    counts
}