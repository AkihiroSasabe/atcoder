use proconio::input;
fn main() {
    input! {
        n: usize
    }
    let mut nn = n - 97;
    let small: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    println!("{}", small[nn]);
}