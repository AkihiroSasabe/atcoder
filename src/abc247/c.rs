use proconio::input;
fn main() {
    input! {
        n: usize
    }

    let answer = saiki(n);
    for i in answer {
        print!("{} ", i);
    }

}

fn saiki(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1]
    }
    
    let mut left = saiki(n-1);
    let mut right = left.clone();
    left.push(n);
    left.append(&mut right);

    return left
}