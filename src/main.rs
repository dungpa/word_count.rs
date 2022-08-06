use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let word_separator = Regex::new(r"([ ,.]+)").expect("Invalid regex");

    if let Ok(lines) = read_lines("sample_50_words.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let splits: Vec<_> = word_separator.split(&ip).into_iter().collect();
                for split in splits {
                    println!("\"{}\"", split);
                }
            }
        }
    }
    println!("Done");
}
