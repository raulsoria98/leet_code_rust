#![allow(dead_code)]

/*

A sentence is a string of single-space separated words where each word consists only of lowercase letters.

A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.

Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.

*/

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut word_count = std::collections::HashMap::new();

    for word in s1.split_whitespace().chain(s2.split_whitespace()) {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(word, _)| word.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_empty_strings() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let result = uncommon_from_sentences(s1, s2);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_one_empty_string() {
        let s1 = "".to_string();
        let s2 = "hello world".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(result.sort(), expected.sort());
    }

    #[test]
    fn test_no_common_words() {
        let s1 = "apple banana".to_string();
        let s2 = "carrot dragonfruit".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec![
            "apple".to_string(),
            "banana".to_string(),
            "carrot".to_string(),
            "dragonfruit".to_string(),
        ];
        assert_eq!(result.sort(), expected.sort());
    }

    #[test]
    fn test_some_common_words() {
        let s1 = "apple banana orange".to_string();
        let s2 = "banana pear grape".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec![
            "apple".to_string(),
            "orange".to_string(),
            "pear".to_string(),
            "grape".to_string(),
        ];
        assert_eq!(result.sort(), expected.sort());
    }

    #[test]
    fn test_all_common_words() {
        let s1 = "apple banana".to_string();
        let s2 = "apple banana".to_string();
        let result = uncommon_from_sentences(s1, s2);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_repeated_words_in_s1() {
        let s1 = "apple apple banana".to_string();
        let s2 = "banana orange".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec!["orange".to_string()];
        assert_eq!(result.sort(), expected.sort());
    }

    #[test]
    fn test_repeated_words_in_s2() {
        let s1 = "apple banana".to_string();
        let s2 = "orange orange banana".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec!["apple".to_string()];
        assert_eq!(result.sort(), expected.sort());
    }

    #[test]
    fn test_complex_case() {
        let s1 = "apple banana orange apple grape".to_string();
        let s2 = "banana pear grape peach".to_string();
        let mut result = uncommon_from_sentences(s1, s2);
        let mut expected = vec![
            "orange".to_string(),
            "pear".to_string(),
            "peach".to_string(),
        ];
        assert_eq!(result.sort(), expected.sort());
    }
}
