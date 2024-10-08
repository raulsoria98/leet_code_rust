#![allow(dead_code, unused_comparisons)]

/*

You are given a string s. You can convert s to a palindrome by adding characters in front of it.

Return the shortest palindrome you can find by performing this transformation.

*/

pub fn shortest_palindrome(s: String) -> String {
    if s.len() <= 0 {
        return s;
    }

    let v: Vec<char> = s.chars().collect();

    let mut left = 0;
    let mut right = s.len() - 1;

    while right >= 0 {
        if v[left] == v[right] {
            left += 1
        }

        if right == 0 {
            break;
        }
        right -= 1;
    }

    let remain = s[left..].to_string();

    if remain.is_empty() {
        return s;
    }

    let rev_remain: String = remain.chars().rev().collect();

    rev_remain + &shortest_palindrome(v[..left].iter().collect()) + &remain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_palindrome_example1() {
        let input = "aacecaaa".to_string();
        let expected = "aaacecaaa".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_example2() {
        let input = "abcd".to_string();
        let expected = "dcbabcd".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_empty() {
        let input = "".to_string();
        let expected = "".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_single_char() {
        let input = "a".to_string();
        let expected = "a".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_already_palindrome() {
        let input = "racecar".to_string();
        let expected = "racecar".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_mixed() {
        let input = "abca".to_string();
        let expected = "acbabca".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }

    #[test]
    fn test_shortest_palindrome_long() {
        let input = "abcde".to_string();
        let expected = "edcbabcde".to_string();
        assert_eq!(shortest_palindrome(input), expected);
    }
}
