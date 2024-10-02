#![allow(dead_code)]

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        if target < 0 {
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            path.push(candidates[i]);
            backtrack(candidates, target - candidates[i], i + 1, path, result);
            path.pop();
        }
    }

    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut candidates = candidates;
    candidates.sort_unstable();

    backtrack(&candidates, target, 0, &mut path, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2_example1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut result = combination_sum2(candidates, target);
        let mut expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_example2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let mut result = combination_sum2(candidates, target);
        let mut expected = vec![vec![1, 2, 2], vec![5]];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_no_solution() {
        let candidates = vec![3, 4, 7];
        let target = 2;
        let result = combination_sum2(candidates, target);
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_empty_input() {
        let candidates = vec![];
        let target = 7;
        let result = combination_sum2(candidates, target);
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_single_candidate_matches() {
        let candidates = vec![7];
        let target = 7;
        let result = combination_sum2(candidates, target);
        let expected = vec![vec![7]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_duplicates_handling() {
        let candidates = vec![2, 2, 2, 2, 2];
        let target = 4;
        let result = combination_sum2(candidates, target);
        let expected = vec![vec![2, 2]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_large_target() {
        let candidates = vec![1, 2, 3, 4, 5];
        let target = 15;
        let result = combination_sum2(candidates, target);
        let expected: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4, 5]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_large_candidate_values() {
        let candidates = vec![100, 200, 300];
        let target = 600;
        let result = combination_sum2(candidates, target);
        let expected = vec![vec![100, 200, 300]];

        assert_eq!(result, expected);
    }
}
