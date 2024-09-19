#![allow(dead_code)]

pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    let reversed: String = x_str.chars().rev().collect();

    x_str == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit() {
        let x = 7;
        assert_eq!(is_palindrome(x), true);
    }

    #[test]
    fn test_two_digit_palindrome() {
        let x = 11;
        assert_eq!(is_palindrome(x), true);
    }

    #[test]
    fn test_two_digit_non_palindrome() {
        let x = 23;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_three_digit_palindrome() {
        let x = 121;
        assert_eq!(is_palindrome(x), true);
    }

    #[test]
    fn test_three_digit_non_palindrome() {
        let x = 123;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_large_palindrome() {
        let x = 12321;
        assert_eq!(is_palindrome(x), true);
    }

    #[test]
    fn test_large_non_palindrome() {
        let x = 12345;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_negative_number() {
        let x = -121;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_zero() {
        let x = 0;
        assert_eq!(is_palindrome(x), true);
    }
}
