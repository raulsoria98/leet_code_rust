use problems::combination_sum2::combination_sum2;

mod problems;

fn main() {
    let res = combination_sum2([10, 1, 2, 7, 6, 1, 5].to_vec(), 8);

    println!("{:?}", res)
}
