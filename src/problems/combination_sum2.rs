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
