#![allow(dead_code)]

/*

A parentheses string is valid if and only if:

    It is the empty string,
    It can be written as AB (A concatenated with B), where A and B are valid strings, or
    It can be written as (A), where A is a valid string.

You are given a parentheses string s. In one move, you can insert a parenthesis at any position of the string.

    For example, if s = "()))", you can insert an opening parenthesis to be "(()))" or a closing parenthesis to be "())))".

Return the minimum number of moves required to make s valid.

*/

pub fn min_add_to_make_valid(s: String) -> i32 {
    let (mut open, mut moves) = (0, 0);

    for ch in s.chars() {
        if ch == '(' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        } else {
            moves += 1;
        }
    }

    moves + open
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "())".to_string();
        assert_eq!(min_add_to_make_valid(s), 1);
    }

    #[test]
    fn test_example_2() {
        let s = "(((".to_string();
        assert_eq!(min_add_to_make_valid(s), 3);
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        assert_eq!(min_add_to_make_valid(s), 0);
    }

    #[test]
    fn test_valid_string() {
        let s = "()()".to_string();
        assert_eq!(min_add_to_make_valid(s), 0);
    }

    #[test]
    fn test_only_closing_parentheses() {
        let s = ")))".to_string();
        assert_eq!(min_add_to_make_valid(s), 3);
    }

    #[test]
    fn test_mixed_parentheses() {
        let s = "())(()".to_string();
        assert_eq!(min_add_to_make_valid(s), 2);
    }

    #[test]
    fn test_longer_string() {
        let s = "((())())((()".to_string();
        assert_eq!(min_add_to_make_valid(s), 2);
    }

    #[test]
    fn test_all_open_parentheses() {
        let s = "(((((((((".to_string();
        assert_eq!(min_add_to_make_valid(s), 9);
    }

    #[test]
    fn test_all_closed_parentheses() {
        let s = ")))))))))".to_string();
        assert_eq!(min_add_to_make_valid(s), 9);
    }
}
