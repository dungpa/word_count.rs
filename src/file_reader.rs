use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use core::result::Result;
use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_into_words(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    let word_separator = Regex::new(r"([ !,;:.]+)").expect("Invalid regex");

    let mut words: Vec<String> = vec![];
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let new_words: Vec<String> = word_separator.split(&ip).map(str::to_string).collect();
            words.extend(new_words);
        }
    }
    words   
}

pub fn read_words<P>(filename: P) -> Result<Vec<String>, String> where P: AsRef<Path>, {
    if let Ok(lines) = read_lines(filename) {
        return Ok(split_into_words(lines));
    }
    return Err("Unable to read file".to_string());
}