use problems::p_1671_minimum_mountain_removals::minimum_mountain_removals;

mod problems;

fn main() {
    let res = minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]);

    println!("{:?}", res)
}
