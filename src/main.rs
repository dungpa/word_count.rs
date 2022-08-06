use std::env;
use std::time::{Instant};
mod file_reader;
mod word_counter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: word_count.exe input_file.txt");
    }
    else {
        let start = Instant::now();
        let input_file_name = &args[1];
        if let Ok(lines) = file_reader::read_lines(input_file_name) {
            let words = word_counter::split_into_words(lines);
            let word_counts = word_counter::count_words(words);

            let duration = start.elapsed();
            println!("Time elapsed in word counting is: {:?}", duration);

            for (key, value) in word_counts {
                println!("{}: {}", key, value);
            }
        }
    }
}
