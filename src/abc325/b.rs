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
    input! {
        n: usize,
    }
    let mut w = vec![];
    let mut x = vec![];
    for i in 0..n {
        input!{
            w_i: usize,
            x_i: usize,
        }
        w.push(w_i);
        x.push(x_i);
    }

    let mut ans = 0;
    for i in 0..24 {
        let mut count = 0;
        let mut aaa = vec![];
        for j in 0..n {
            let start_time = (x[j] + i) % 24;
            if 9 <= start_time && start_time <= 17 {
                count += w[j];
            }
            aaa.push(start_time);
        }
        // println!("i = {i}, aaa = {:?}, count = {count}", aaa);
        ans = max(count, ans);
    }
    println!("{}", ans);
}