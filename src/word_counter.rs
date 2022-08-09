use itertools::Itertools;

pub fn count_words(words: Vec<String>) -> impl Iterator<Item=(usize, String)> {
    let counts = 
        words.into_iter()
             .filter_map(|s| { if s.trim().is_empty() { None } else { Some((s.to_lowercase(), s)) } })
             .into_group_map()
             .into_iter()
             .map (|(key, value)| (value.len(), key))
             .sorted();
    counts
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_words_works_on_empty_input() {
        let input = vec![];
        let expected = vec![];
        let actual = crate::word_counter::count_words(input);
        itertools::assert_equal(actual, expected);
    }

    #[test]
    fn count_words_works_on_words_with_single_occurences() {
        let input = vec!["Hello".into(), "world".into()];
        let expected = vec![(1, "hello".into()), (1, "world".into())];
        let actual = crate::word_counter::count_words(input);
        itertools::assert_equal(actual, expected);
    }

    #[test]
    fn count_words_works_on_words_with_multiple_occurences() {
        let input = vec!["Hello".into(), "world".into(), "world".into()];
        let expected = vec![(1, "hello".into()), (2, "world".into())];
        let actual = crate::word_counter::count_words(input);
        itertools::assert_equal(actual, expected);
    }
}