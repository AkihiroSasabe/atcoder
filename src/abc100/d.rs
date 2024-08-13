#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-08-13 20:15-20:35 (20min)
    input! {
        n: usize,
        m: usize,
    }
    // |xi| <= 10^10
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    let mut xyz = vec![];
    
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
            zi: isize,
        }
        x.push(xi);
        y.push(yi);
        z.push(zi);
        xyz.push(vec![xi, yi, zi]);
    }

    // 8パターン考えれば良さそう
    // xi: -, +
    // yi: -, +
    // zi: -, +

    let mut sums = vec![vec![]; 1<<3];
    let mut ans = 0;
    for mask in 0..1<<3 {
        for i in 0..n {
            let mut sum = 0;
            for j in 0..3 {
                if mask & (1 << j) != 0 {
                    sum += xyz[i][j];
                }
                else {
                    sum -= xyz[i][j];
                }
            }
            sums[mask].push(sum);
        }
        // println!("sums[{mask}] = {:?}", sums[mask]);
        sums[mask].sort();
        sums[mask].reverse();
        let mut temp = 0;
        for i in 0..m {
            temp += sums[mask][i];
        }
        ans = max(ans, temp);
    }
    println!("{}", ans);

    



}