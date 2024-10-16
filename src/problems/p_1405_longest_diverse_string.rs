#![allow(dead_code)]

/*

A string s is called happy if it satisfies the following conditions:

    s only contains the letters 'a', 'b', and 'c'.
    s does not contain any of "aaa", "bbb", or "ccc" as a substring.
    s contains at most a occurrences of the letter 'a'.
    s contains at most b occurrences of the letter 'b'.
    s contains at most c occurrences of the letter 'c'.

Given three integers a, b, and c, return the longest possible happy string. If there are multiple longest happy strings,
return any of them. If there is no such string, return the empty string "".

A substring is a contiguous sequence of characters within a string.

*/

pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    use std::collections::BinaryHeap;

    let mut v: Vec<char> = vec![];

    let mut max_queue: BinaryHeap<(i32, char)> = BinaryHeap::new();
    if a > 0 {
        max_queue.push((a, 'a'));
    }
    if b > 0 {
        max_queue.push((b, 'b'));
    }
    if c > 0 {
        max_queue.push((c, 'c'));
    }

    while let Some(mut c) = max_queue.pop() {
        if v.len() >= 2 && v[v.len() - 1] == c.1 && v[v.len() - 2] == c.1 {
            if let Some(mut next_c) = max_queue.pop() {
                v.push(next_c.1);
                next_c.0 -= 1;
                if next_c.0 > 0 {
                    max_queue.push(next_c);
                }

                max_queue.push(c);
            } else {
                break;
            }
        } else {
            v.push(c.1);
            c.0 -= 1;

            if c.0 > 0 {
                max_queue.push(c);
            }
        }
    }

    v.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Función auxiliar para verificar si una cadena es feliz
    fn is_happy_string(s: &str, a: i32, b: i32, c: i32) -> bool {
        let mut count_a = 0;
        let mut count_b = 0;
        let mut count_c = 0;

        for window in s.as_bytes().windows(3) {
            if window == b"aaa" || window == b"bbb" || window == b"ccc" {
                return false;
            }
        }

        for ch in s.chars() {
            match ch {
                'a' => count_a += 1,
                'b' => count_b += 1,
                'c' => count_c += 1,
                _ => return false, // caracteres inválidos
            }
        }

        count_a <= a && count_b <= b && count_c <= c
    }

    #[test]
    fn test_example_1() {
        let a = 1;
        let b = 1;
        let c = 7;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_example_2() {
        let a = 7;
        let b = 1;
        let c = 0;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_edge_case_zero() {
        let a = 0;
        let b = 0;
        let c = 0;
        let result = longest_diverse_string(a, b, c);
        assert_eq!(result, "");
    }

    #[test]
    fn test_large_values() {
        let a = 100;
        let b = 100;
        let c = 100;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert!(result.len() == 300);
    }

    #[test]
    fn test_only_a() {
        let a = 5;
        let b = 0;
        let c = 0;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 2);
        assert!(result.contains("a"));
    }

    #[test]
    fn test_only_b() {
        let a = 0;
        let b = 5;
        let c = 0;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 2);
        assert!(result.contains("b"));
    }

    #[test]
    fn test_only_c() {
        let a = 0;
        let b = 0;
        let c = 5;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 2);
        assert!(result.contains("c"));
    }

    #[test]
    fn test_a_b_equal() {
        let a = 3;
        let b = 3;
        let c = 0;
        let result = longest_diverse_string(a, b, c);
        assert!(is_happy_string(&result, a, b, c));
        assert_eq!(result.len(), 6);
    }
}
