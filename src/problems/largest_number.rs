#![allow(dead_code)]

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut str_nums: Vec<String> = nums.iter().map(|n| n.to_string()).collect();

    str_nums.sort_unstable_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));

    if str_nums[0] == "0" {
        return "0".to_string();
    }

    str_nums.concat()
}
