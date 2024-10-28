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
use rand::Rng;
fn main() {
    // 2024-10-22 20:02-20:34 (32min)
    input! {
        n: usize,
        a: [isize; n],
    }

    // 必要な操作回数
    // Σ[0,i] a[i] != 0
    // (Σ[0,i] a[i]) * (Σ[0,i+1] a[i]) < 0

    // p, m, p, m, ...
    // |Σ[0,i] a[i]| < |a[i+1]|

    let cand = solve(&a);
    let cand2 = solve2(&a);

    let ans = min(cand, cand2);
    println!("{}", ans);
}


fn solve(a: &Vec<isize>) -> isize {
    let n = a.len();
    // 愚直にやるだけ
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..n {
        let pre_sum = sum;
        sum += a[i];
        // if i == 0 {continue}

        if i % 2 == 0 {
            // plusじゃないと駄目
            if sum <= 0 {
                ans += sum.abs() + 1;
                sum = 1;
            }
        }
        else {
            // minus じゃないと駄目
            if sum >= 0 {
                ans += sum + 1;
                sum = -1;
            }
        }
    }
    return ans
}

fn solve2(a: &Vec<isize>) -> isize {
    let n = a.len();
    // 愚直にやるだけ
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..n {
        let pre_sum = sum;
        sum += a[i];
        // if i == 0 {continue}

        if i % 2 == 0 {
            // minus じゃないと駄目
            if sum >= 0 {
                ans += sum + 1;
                sum = -1;
            }
        }
        else {
            // plusじゃないと駄目
            if sum <= 0 {
                ans += sum.abs() + 1;
                sum = 1;
            }
        }
    }
    return ans
}