#![allow(dead_code)]

/*

You are given a 0-indexed string s of even length n. The string consists of exactly n / 2 opening brackets '['
and n / 2 closing brackets ']'.

A string is called balanced if and only if:

    It is the empty string, or
    It can be written as AB, where both A and B are balanced strings, or
    It can be written as [C], where C is a balanced string.

You may swap the brackets at any two indices any number of times.

Return the minimum number of swaps to make s balanced.

*/

pub fn min_swaps(s: String) -> i32 {
    let mut open = 0;

    for ch in s.chars() {
        if ch == '[' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        }
    }

    (open + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(min_swaps("".to_string()), 0);
    }

    #[test]
    fn test_no_swaps_needed() {
        assert_eq!(min_swaps("[[]]".to_string()), 0);
        assert_eq!(min_swaps("[[][]]".to_string()), 0);
    }

    #[test]
    fn test_one_swap_needed() {
        assert_eq!(min_swaps("][][".to_string()), 1);
        assert_eq!(min_swaps("[]][".to_string()), 1);
    }

    #[test]
    fn test_two_swaps_needed() {
        assert_eq!(min_swaps("]]][[[".to_string()), 2);
    }

    #[test]
    fn test_complex_case() {
        assert_eq!(min_swaps("[]][[]][[".to_string()), 1);
    }

    #[test]
    fn test_mixed_balanced_case() {
        assert_eq!(min_swaps("[[[]]][[]]".to_string()), 0);
    }
}
