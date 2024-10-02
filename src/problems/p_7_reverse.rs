#![allow(dead_code)]

pub fn reverse(x: i32) -> i32 {
    if x == i32::MIN {
        return 0;
    }

    let x_str = x.abs().to_string();
    let reversed: String = x_str.chars().rev().collect();

    let res = match reversed.parse::<i32>() {
        Ok(val) => {
            if x < 0 {
                -val
            } else {
                val
            }
        }
        Err(_) => 0,
    };

    res
}

#[cfg(test)]
mod tests {
    use std::i32;

    use super::*;

    #[test]
    fn test_positive_number() {
        let x = 123;
        let result = reverse(x);
        let expected = 321;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negative_number() {
        let x = -456;
        let result = reverse(x);
        let expected = -654;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_with_trailing_zeroes() {
        let x = 120;
        let result = reverse(x);
        let expected = 21;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_digit() {
        let x = 7;
        let result = reverse(x);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_number() {
        let x = 123456789;
        let result = reverse(x);
        let expected = 987654321;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero() {
        let x = 0;
        let result = reverse(x);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_overflow() {
        let x = i32::MAX;
        let result = reverse(x);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minimum_negative() {
        let x = i32::MIN;
        let result = reverse(x);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
