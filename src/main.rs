use problems::longest_palindrome::longest_palindrome;

mod problems;

fn main() {
    let res = longest_palindrome("abab".to_string());

    println!("{:?}", res)
}
