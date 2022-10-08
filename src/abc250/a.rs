use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize
    }

    let mut count = 4;
    if r == 1 {
        count -= 1;
    }

    if c == 1 {
        count -= 1;
    }

    if r == h {
        count -= 1;
    }

    if c == w {
        count -= 1;
    }

    println!("{}", count);

}