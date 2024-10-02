use problems::p_1331_array_rank_transform::array_rank_transform;

mod problems;

fn main() {
    let res = array_rank_transform(vec![4, 2, 1, 5, 2]);

    println!("{:?}", res)
}
