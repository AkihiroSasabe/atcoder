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
        k: usize,
        a: [usize; k],
    }
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        input! {
            x_i: f64,
            y_i: f64
        }
        x.push(x_i);
        y.push(y_i);
    }

    // let INF: f64 = 100_000_000_000.0;

    let mut whole_dists = vec![];
    for i in 0..n {
        let mut dists = vec![];
        for j in 0..k {
            dists.push((x[a[j] - 1] - x[i]) * (x[a[j] - 1] - x[i]) + (y[a[j] - 1] - y[i]) * (y[a[j] - 1] - y[i]));
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        whole_dists.push(dists[0]);
    }
    whole_dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}", whole_dists[n-1].sqrt());
}