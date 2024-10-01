#![allow(dead_code)]

pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut remainder_count = vec![0; k as usize];

    for &i in &arr {
        let remainder = ((i % k) + k) % k;
        remainder_count[remainder as usize] += 1;
    }

    if remainder_count[0] % 2 != 0 {
        return false;
    }

    for i in 1..k {
        let compliment = k - i;
        if remainder_count[i as usize] != remainder_count[compliment as usize] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
        let k = 5;
        assert_eq!(can_arrange(arr, k), true);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 7;
        assert_eq!(can_arrange(arr, k), true);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        assert_eq!(can_arrange(arr, k), false);
    }

    #[test]
    fn test_negative_numbers() {
        let arr = vec![-1, -2, -3, -4, -5, -6];
        let k = 7;
        assert_eq!(can_arrange(arr, k), true);
    }

    #[test]
    fn test() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let k = 7;
        assert_eq!(can_arrange(arr, k), false);
    }

    #[test]
    fn test_all_elements_same() {
        let arr = vec![5, 5, 5, 5];
        let k = 5;
        assert_eq!(can_arrange(arr, k), true);
    }

    #[test]
    fn test_large_k() {
        let arr = vec![10, 20, 30, 40, 50, 60];
        let k = 100;
        assert_eq!(can_arrange(arr, k), false);
    }
}
