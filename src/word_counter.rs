use itertools::Itertools;

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