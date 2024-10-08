#![allow(dead_code)]

/*

Given a string s, return the longest palindromic substring in s.

*/

pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len <= 1 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut max_len = 1;

    fn expand_around_center(chars: &[char], mut left: usize, mut right: usize) -> (usize, usize) {
        while left > 0 && right < chars.len() && chars[left - 1] == chars[right] {
            left -= 1;
            right += 1;
        }
        (left, right)
    }

    for i in 0..len {
        let (l1, r1) = expand_around_center(&chars, i, i + 1);
        if r1 - l1 > max_len {
            start = l1;
            max_len = r1 - l1;
        }

        if i + 1 < len && chars[i] == chars[i + 1] {
            let (l2, r2) = expand_around_center(&chars, i, i + 2);
            if r2 - l2 > max_len {
                start = l2;
                max_len = r2 - l2;
            }
        }
    }

    s[start..start + max_len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = String::from("babad");
        let output = longest_palindrome(input);
        assert!(output == "bab" || output == "aba");
    }

    #[test]
    fn test_example2() {
        let input = String::from("cbbd");
        let output = longest_palindrome(input);
        assert_eq!(output, "bb");
    }

    #[test]
    fn test_single_character() {
        let input = String::from("a");
        let output = longest_palindrome(input);
        assert_eq!(output, "a");
    }

    #[test]
    fn test_two_same_characters() {
        let input = String::from("aa");
        let output = longest_palindrome(input);
        assert_eq!(output, "aa");
    }

    #[test]
    fn test_no_palindrome() {
        let input = String::from("abcde");
        let output = longest_palindrome(input);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_palindrome_entire_string() {
        let input = String::from("racecar");
        let output = longest_palindrome(input);
        assert_eq!(output, "racecar");
    }

    #[test]
    fn test_even_length_palindrome() {
        let input = String::from("abccba");
        let output = longest_palindrome(input);
        assert_eq!(output, "abccba");
    }

    #[test]
    fn test_large_string() {
        let input = "a".repeat(1000);
        let output = longest_palindrome(input.clone());
        assert_eq!(output.len(), 1000);
    }

    #[test]
    fn test_abab() {
        let input = String::from("abab");
        let output = longest_palindrome(input);
        assert_eq!(output, "aba");
    }
}
