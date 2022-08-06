use regex::Regex;
mod file_reader;

fn main() {
    let word_separator = Regex::new(r"([ ,.]+)").expect("Invalid regex");

    if let Ok(lines) = file_reader::read_lines("sample_50_words.txt") {
        let mut words: Vec<String> = vec![];
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let new_words: Vec<String> = word_separator.split(&ip).map(str::to_string).collect();
                words.extend(new_words);
            }
        }

        for word in words {
            println!("\"{}\"", word);
        }
    }
    println!("Done");
}
