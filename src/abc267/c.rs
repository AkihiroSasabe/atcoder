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
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut a_cum = vec![0; n];
    a_cum[0] = a[0];
    for i in 1..n {
        a_cum[i] = a_cum[i-1] + a[i];
    }

    let mut b_cum = vec![0; n];
    b_cum[0] = a_cum[m-1];
    for i in 1..n {
        if i+m-1 >= n {break}
        b_cum[i] = a_cum[i+m-1] - a_cum[i-1];
    }

    let mut weighted_a_cum = vec![0; n];
    weighted_a_cum[0] = 1 * a[0];
    for i in 1..n {
        weighted_a_cum[i] = weighted_a_cum[i-1] + (i as isize +1) * a[i];
    }

    let mut weighted_b_cum = vec![0; n];
    weighted_b_cum[0] = weighted_a_cum[m-1];
    for i in 1..n {
        if i+m-1 >= n {break}
        weighted_b_cum[i] = weighted_a_cum[i+m-1] - weighted_a_cum[i-1] - (i as isize) * b_cum[i];
    }

    let mut ans = (-1) * (1 << 60);
    for i in 0..(n-m+1) {
        ans = max(ans, weighted_b_cum[i]);
    }
    println!("{}", ans);


}