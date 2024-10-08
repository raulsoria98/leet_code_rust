#![allow(dead_code)]

/*

Given an array of integers arr, replace each element with its rank.

The rank represents how large the element is. The rank has the following rules:

    Rank is an integer starting from 1.
    The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
    Rank should be as small as possible.

*/

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut sorted = arr.clone();
    sorted.sort_unstable();

    let mut rank_map = std::collections::HashMap::new();
    let mut rank = 1;

    for &num in &sorted {
        rank_map.entry(num).or_insert(rank);

        if *rank_map.get(&num).unwrap() == rank {
            rank += 1
        }
    }

    arr.into_iter()
        .map(|num| *rank_map.get(&num).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![10];
        let expected = vec![1];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_sorted_array() {
        let arr = vec![1, 2, 3];
        let expected = vec![1, 2, 3];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let arr = vec![3, 2, 1];
        let expected = vec![3, 2, 1];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_array_with_duplicates() {
        let arr = vec![40, 10, 20, 30, 20];
        let expected = vec![4, 1, 2, 3, 2];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_all_elements_equal() {
        let arr = vec![7, 7, 7];
        let expected = vec![1, 1, 1];
        assert_eq!(array_rank_transform(arr), expected);
    }

    #[test]
    fn test_large_array() {
        let arr = vec![100, 50, 50, 75, 100];
        let expected = vec![3, 1, 1, 2, 3];
        assert_eq!(array_rank_transform(arr), expected);
    }
}
