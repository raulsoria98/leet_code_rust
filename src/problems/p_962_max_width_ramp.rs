#![allow(dead_code)]

/*

A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j].
The width of such a ramp is j - i.

Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.

*/

pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut max_width = 0;

    for i in 0..nums.len() {
        if stack.is_empty() || nums[i] < nums[*stack.last().unwrap()] {
            stack.push(i);
        }
    }

    for j in (0..nums.len()).rev() {
        while !stack.is_empty() && nums[*stack.last().unwrap()] <= nums[j] {
            let i = stack.pop().unwrap();
            max_width = max_width.max(j as i32 - i as i32);
        }
    }

    max_width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let nums = vec![];
        assert_eq!(max_width_ramp(nums), 0);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        assert_eq!(max_width_ramp(nums), 0);
    }

    #[test]
    fn test_no_ramp() {
        let nums = vec![5, 4, 3, 2, 1];
        assert_eq!(max_width_ramp(nums), 0);
    }

    #[test]
    fn test_simple_ramp() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(max_width_ramp(nums), 4);
    }

    #[test]
    fn test_decreasing_and_increasing() {
        let nums = vec![6, 0, 8, 2, 1, 5];
        assert_eq!(max_width_ramp(nums), 4);
    }

    #[test]
    fn test_max_ramp_at_end() {
        let nums = vec![9, 8, 1, 0, 1, 9];
        assert_eq!(max_width_ramp(nums), 5);
    }

    #[test]
    fn test_multiple_ramps() {
        let nums = vec![3, 4, 2, 5, 0, 6];
        assert_eq!(max_width_ramp(nums), 5);
    }

    #[test]
    fn test_ramp_with_equal_values() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(max_width_ramp(nums), 3);
    }

    #[test]
    fn test_ramp_with_large_numbers() {
        let nums = vec![1, 100000, 1000000, 10000000];
        assert_eq!(max_width_ramp(nums), 3);
    }

    #[test]
    fn test_ramp_with_duplicates() {
        let nums = vec![10, 9, 8, 7, 7, 7];
        assert_eq!(max_width_ramp(nums), 2);
    }
}
