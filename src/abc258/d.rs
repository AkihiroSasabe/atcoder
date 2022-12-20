use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize
        }
        a.push(a_i);
        b.push(b_i);
    }
    let INF = std::usize::MAX;
    let mut ans = INF;
    let mut first_term = 0;
    for i in 0..n {
        first_term += a[i] + b[i];
        if x < i + 1 {continue}
        ans = min(ans, first_term + (x-1-i)* b[i]);
    }
    println!("{}", ans);

}