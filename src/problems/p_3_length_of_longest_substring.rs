#![allow(dead_code)]

/*

Given a string s, find the length of the longest substring without repeating characters.

*/

pub fn length_of_longest_substring(s: String) -> i32 {
    let characters: Vec<char> = s.chars().collect();
    let mut longest = "".to_string();
    let mut current = "".to_string();
    let mut starting_char = 0;

    while starting_char != characters.len() {
        for i in starting_char..=characters.len() - 1 {
            if let Some(&curr_char) = characters.get(i) {
                if !current.contains(curr_char) {
                    current.push(curr_char);
                } else {
                    break;
                }
            }
        }
        if current.len() > longest.len() {
            longest = current;
        }
        current = "".to_string();
        starting_char += 1;
    }

    longest.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_character() {
        let input = "a".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_unique_characters() {
        let input = "abcde".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_repeating_characters() {
        let input = "abcabcbb".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_mixed_characters() {
        let input = "pwwkew".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_large_string() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_large_string_with_repeats() {
        let input = "abcdabcdefgabcdefg".to_string();
        let result = length_of_longest_substring(input);
        assert_eq!(result, 7);
    }
}
