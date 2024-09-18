#![allow(dead_code)]

pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    let reversed: String = x_str.chars().rev().collect();

    x_str == reversed
}
