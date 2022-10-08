use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let t1 = x / (a+c);
    let l1 = t1 * b * a + min((x-t1 * (a+c)), a) * b;
    let t2 = x / (d+f);
    let l2 = t2 * e * d + min((x-t2 * (d+f)), d) * e;

    if l1 > l2 {
        println!("Takahashi");
    }
    if l1 < l2 {
        println!("Aoki");
    }
    if l1 == l2 {
        println!("Draw");
    }
}