use regex::Regex;
use itertools::Itertools;
mod file_reader;

fn main() {
    let word_separator = Regex::new(r"([ ,.]+)").expect("Invalid regex");

    if let Ok(lines) = file_reader::read_lines("sample_50_words.txt") {
        let mut words: Vec<String> = vec![];
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let new_words: Vec<String> = word_separator.split(&ip).map(str::to_lowercase).collect();
                words.extend(new_words);
            }
        }

        // It is inefficient to clone the strings, we live with it for now.
        let hash_map = words.into_iter().map(|s| (s.clone(), s.clone())).into_group_map();

        for (key, value) in hash_map.into_iter() {
            println!("{}: {}", key, value.len());
        }
    }
    println!("Done");
}
