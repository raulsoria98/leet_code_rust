#![allow(dead_code)]

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index, i as i32];
        }

        map.insert(num, i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_case2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_case3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_case4() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_two_sum_case5() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![]); // No solution expected
    }
}
