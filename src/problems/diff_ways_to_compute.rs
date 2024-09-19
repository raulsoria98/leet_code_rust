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
