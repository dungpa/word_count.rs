mod file_reader;
mod word_counter;

fn main() {
    if let Ok(lines) = file_reader::read_lines("sample_50_words.txt") {
        let words = word_counter::split_into_words(lines);
        let word_counts = word_counter::count_words(words);

        for (key, value) in word_counts.into_iter() {
            println!("{}: {}", key, value.len());
        }
    }
}
