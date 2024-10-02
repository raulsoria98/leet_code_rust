#![allow(dead_code)]

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    if expression.chars().all(|c| c.is_digit(10)) {
        return vec![expression.parse().unwrap()];
    }

    let mut results = Vec::new();

    for (i, c) in expression.chars().enumerate() {
        if c == '+' || c == '-' || c == '*' {
            let left = diff_ways_to_compute(expression[0..i].to_string());
            let right = diff_ways_to_compute(expression[i + 1..].to_string());

            for &l in &left {
                for &r in &right {
                    match c {
                        '+' => results.push(l + r),
                        '-' => results.push(l - r),
                        '*' => results.push(l * r),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let expression = String::from("3");
        let result = diff_ways_to_compute(expression);
        let expected = vec![3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_addition() {
        let expression = String::from("2+3");
        let result = diff_ways_to_compute(expression);
        let expected = vec![5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_subtraction() {
        let expression = String::from("5-2");
        let result = diff_ways_to_compute(expression);
        let expected = vec![3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_multiplication() {
        let expression = String::from("4*3");
        let result = diff_ways_to_compute(expression);
        let expected = vec![12];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_operations() {
        let expression = String::from("2-1-1");
        let mut result = diff_ways_to_compute(expression);
        let mut expected = vec![0, 2];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_more_mixed_operations() {
        let expression = String::from("2*3-4*5");
        let mut result = diff_ways_to_compute(expression);
        let mut expected = vec![-34, -14, -10, -10, 10];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_operations() {
        let expression = String::from("2+3*4-5");
        let mut result = diff_ways_to_compute(expression);
        let mut expected = vec![-5, -1, 9, 9, 15];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_operator() {
        let expression = String::from("123");
        let result = diff_ways_to_compute(expression);
        let expected = vec![123];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_expression() {
        let expression = String::from("10+5*3-8");
        let mut result = diff_ways_to_compute(expression);
        let mut expected = vec![-75, -15, 17, 17, 37];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
