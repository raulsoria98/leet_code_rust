#![allow(dead_code)]

/*

You may recall that an array arr is a mountain array if and only if:

    arr.length >= 3
    There exists some index i (0-indexed) with 0 < i < arr.length - 1 such that:
        arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
        arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

Given an integer array nums​​​, return the minimum number of elements to remove to make nums​​​ a mountain array.

*/

pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut increasing = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                increasing[i] = increasing[i].max(increasing[j] + 1);
            }
        }
    }

    let mut decreasing = vec![1; n];
    for i in (0..n - 1).rev() {
        for j in (i + 1)..n {
            if nums[i] > nums[j] {
                decreasing[i] = decreasing[i].max(decreasing[j] + 1);
            }
        }
    }

    let mut max_mountain_len = 0;
    for i in 1..n - 1 {
        if increasing[i] > 1 && decreasing[i] > 1 {
            max_mountain_len = max_mountain_len.max(increasing[i] + decreasing[i] - 1);
        }
    }

    (n as i32) - max_mountain_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_mountain() {
        let nums = vec![1, 3, 2];
        assert_eq!(minimum_mountain_removals(nums), 0);
    }

    #[test]
    fn test_remove_to_form_mountain() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        assert_eq!(minimum_mountain_removals(nums), 3);
    }

    #[test]
    fn test_all_increasing() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(minimum_mountain_removals(nums), 5);
    }

    #[test]
    fn test_all_decreasing() {
        let nums = vec![5, 4, 3, 2, 1];
        assert_eq!(minimum_mountain_removals(nums), 5);
    }

    #[test]
    fn test_multiple_peaks() {
        let nums = vec![1, 3, 1, 4, 5, 3, 2];
        assert_eq!(minimum_mountain_removals(nums), 1);
    }

    #[test]
    fn test_large_mountain() {
        let nums = vec![1, 2, 3, 4, 5, 4, 3, 2, 1];
        assert_eq!(minimum_mountain_removals(nums), 0);
    }

    #[test]
    fn test_min_length() {
        let nums = vec![1, 2, 1];
        assert_eq!(minimum_mountain_removals(nums), 0);
    }

    #[test]
    fn test_duplicate_peaks() {
        let nums = vec![1, 2, 2, 3, 2, 2, 1];
        assert_eq!(minimum_mountain_removals(nums), 2);
    }

    #[test]
    fn test_alternating_values() {
        let nums = vec![1, 5, 1, 5, 1, 5, 1];
        assert_eq!(minimum_mountain_removals(nums), 4);
    }

    #[test]
    fn test_longest_possible_mountain() {
        let nums = vec![1, 3, 5, 4, 2, 1, 7, 8, 9];
        assert_eq!(minimum_mountain_removals(nums), 3);
    }
}
