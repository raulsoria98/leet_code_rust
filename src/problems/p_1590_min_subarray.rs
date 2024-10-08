#![allow(dead_code)]

/*

Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the
remaining elements is divisible by p. It is not allowed to remove the whole array.

Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.

A subarray is defined as a contiguous block of elements in the array.

*/

fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let p = p as i64;
    let remainder = total_sum % p;

    if remainder == 0 {
        return 0;
    }

    let mut prefix_sum = 0_i64;
    let mut min_len = nums.len() as i32;
    let mut seen_remainders = std::collections::HashMap::new();
    seen_remainders.insert(0, -1);

    for (i, &num) in nums.iter().enumerate() {
        prefix_sum = (prefix_sum + num as i64) % p;
        let target_remainder = (prefix_sum - remainder + p) % p;

        if let Some(&prev_index) = seen_remainders.get(&target_remainder) {
            min_len = min_len.min(i as i32 - prev_index);
        }

        seen_remainders.insert(prefix_sum % p, i as i32);
    }

    if min_len == nums.len() as i32 {
        -1
    } else {
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        assert_eq!(min_subarray(nums, p), 1);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        assert_eq!(min_subarray(nums, p), 2);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3];
        let p = 3;
        assert_eq!(min_subarray(nums, p), 0);
    }

    #[test]
    fn test_no_subarray_possible() {
        let nums = vec![1, 2, 3, 4];
        let p = 5;
        assert_eq!(min_subarray(nums, p), 0);
    }

    #[test]
    fn test_no_valid_subarray() {
        let nums = vec![8, 11, 13];
        let p = 10;
        assert_eq!(min_subarray(nums, p), -1);
    }

    #[test]
    fn test_single_element_subarray() {
        let nums = vec![7, 1, 2, 6];
        let p = 8;
        assert_eq!(min_subarray(nums, p), 0);
    }

    #[test]
    fn test_empty_subarray() {
        let nums = vec![12, 9, 18];
        let p = 3;
        assert_eq!(min_subarray(nums, p), 0);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1; 100000];
        let p = 100000;
        assert_eq!(min_subarray(nums, p), 0);
    }

    #[test]
    fn test_large_number_subarray() {
        let nums = vec![1000000000, 1, 2, 3, 4];
        let p = 1000000007;
        assert_eq!(min_subarray(nums, p), 1);
    }

    #[test]
    fn test_three_large_number_subarray() {
        let nums = vec![1000000000, 1000000000, 1000000000];
        let p = 3;
        assert_eq!(min_subarray(nums, p), 0);
    }
}
