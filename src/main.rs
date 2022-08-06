use regex::Regex;
mod file_reader;

fn main() {
    let word_separator = Regex::new(r"([ ,.]+)").expect("Invalid regex");

    if let Ok(lines) = file_reader::read_lines("sample_50_words.txt") {
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
