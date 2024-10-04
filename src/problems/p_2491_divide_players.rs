#![allow(dead_code)]

pub fn divide_players(skill: Vec<i32>) -> i64 {
    let n = skill.len();
    let mut skill = skill;
    skill.sort_unstable();

    let mut sum = 0_i64;

    let mut i = 0_usize;
    let mut j = n - 1;
    let team_skill = skill[i] + skill[j];
    while i < n / 2 {
        if team_skill != skill[i] + skill[j] {
            return -1;
        }

        sum += skill[i] as i64 * skill[j] as i64;

        i += 1;
        j -= 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let skill = vec![3, 2, 5, 1, 3, 4];
        assert_eq!(divide_players(skill), 22);
    }

    #[test]
    fn test_example_2() {
        let skill = vec![3, 4];
        assert_eq!(divide_players(skill), 12);
    }

    #[test]
    fn test_example_3() {
        let skill = vec![1, 1, 2, 3];
        assert_eq!(divide_players(skill), -1);
    }

    #[test]
    fn test_edge_case_even_sum() {
        let skill = vec![6, 4, 4, 6];
        assert_eq!(divide_players(skill), 48);
    }

    #[test]
    fn test_large_input() {
        let skill = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(divide_players(skill), 110); // Imposible to divide correctly
    }
}
