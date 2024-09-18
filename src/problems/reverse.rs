#![allow(dead_code)]

pub fn reverse(x: i32) -> i32 {
    let x_str = x.abs().to_string();
    let reversed: String = x_str.chars().rev().collect();

    let res = match reversed.parse::<i32>() {
        Ok(val) => {
            if x < 0 {
                -val
            } else {
                val
            }
        }
        Err(_) => 0,
    };

    res
}
