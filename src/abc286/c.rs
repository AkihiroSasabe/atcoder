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
        n: usize,
        a: usize,
        b: usize,
        mut s: Chars
    }
    let mut ans = std::usize::MAX;

    // a_num;
    let mut a_num = 0;
    for a_num in 0..n {
        let mut b_num = 0;
        for i in 0..n/2 {
            let first_i = i + a_num;
            let last_i = n - 1 - i + a_num;
            if s[first_i] != s[last_i] {
                b_num += 1;
            }
        }
        ans = min(ans, a_num * a + b_num * b);
        s.push(s[a_num]);
    }


    println!("{}", ans);
}