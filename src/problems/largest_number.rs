#![allow(dead_code)]

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut str_nums: Vec<String> = nums.iter().map(|n| n.to_string()).collect();

    str_nums.sort_unstable_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));

    if str_nums[0] == "0" {
        return "0".to_string();
    }

    str_nums.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit_numbers() {
        let nums = vec![3, 30, 34, 5, 9];
        let result = largest_number(nums);
        let expected = "9534330".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_zeroes() {
        let nums = vec![0, 0, 0];
        let result = largest_number(nums);
        let expected = "0".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_numbers() {
        let nums = vec![10, 2];
        let result = largest_number(nums);
        let expected = "210".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![823, 8238];
        let result = largest_number(nums);
        let expected = "8238823".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_case() {
        let nums = vec![1, 11, 111];
        let result = largest_number(nums);
        let expected = "111111".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_number() {
        let nums = vec![1];
        let result = largest_number(nums);
        let expected = "1".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_repeated_numbers() {
        let nums = vec![9, 9, 9, 9];
        let result = largest_number(nums);
        let expected = "9999".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![121, 12];
        let result = largest_number(nums);
        let expected = "12121".to_string();
        assert_eq!(result, expected);
    }
}
