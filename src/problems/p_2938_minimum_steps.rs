#![allow(dead_code)]

/*

There are n balls on a table, each ball has a color black or white.

You are given a 0-indexed binary string s of length n, where 1 and 0 represent black and white balls, respectively.

In each step, you can choose two adjacent balls and swap them.

Return the minimum number of steps to group all the black balls to the right and all the white balls to the left.

*/

pub fn minimum_steps(s: String) -> i64 {
    let (mut black, mut count) = (0, 0);

    for c in s.chars() {
        if c == '1' {
            black += 1;
        } else {
            count += black;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_white() {
        let s = "00000".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_all_black() {
        let s = "11111".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_alternating_white_black() {
        let s = "010101".to_string();
        assert_eq!(minimum_steps(s), 3);
    }

    #[test]
    fn test_white_black_left_right() {
        let s = "000111".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_mixed_order() {
        let s = "110100".to_string();
        assert_eq!(minimum_steps(s), 8);
    }

    #[test]
    fn test_single_white_ball() {
        let s = "0".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_single_black_ball() {
        let s = "1".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        assert_eq!(minimum_steps(s), 0);
    }

    #[test]
    fn test_mostly_black_with_one_white_in_middle() {
        let s = "1110111".to_string();
        assert_eq!(minimum_steps(s), 3);
    }

    #[test]
    fn test_mostly_white_with_one_black_in_middle() {
        let s = "0001000".to_string();
        assert_eq!(minimum_steps(s), 3);
    }

    #[test]
    fn test_black_surrounded_by_white() {
        let s = "000100".to_string();
        assert_eq!(minimum_steps(s), 2);
    }
}
