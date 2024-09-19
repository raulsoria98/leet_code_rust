use problems::diff_ways_to_compute::diff_ways_to_compute;

mod problems;

fn main() {
    let res = diff_ways_to_compute("2-1-1".to_string());

    println!("{:?}", res)
}
