use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    // let mut ans = 1 << 60;
    let ans = min(x*n, y * (n / 3) + x * (n % 3));
    println!("{}", ans);


}