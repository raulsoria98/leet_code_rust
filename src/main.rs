use problems::shortest_palindrome::shortest_palindrome;

mod problems;

fn main() {
    let res = shortest_palindrome("adcba".to_string());

    println!("{:?}", res)
}
