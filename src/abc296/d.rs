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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: u128,
        m: u128,
    }

    if m > n * n {
        println!("-1");
        return
    }
    // a <= b とする
    let mut a: u128 = 1;
    let mut ans: u128 = n*n;
    while (a-1) * (a-1) <= m {
        let b = if m % a == 0 {
            m / a
        }
        else {
            m / a + 1
        };
        if b <= n {
            ans = min(ans, a * b);
        }
        a += 1;
    }
    println!("{}", ans);
}
