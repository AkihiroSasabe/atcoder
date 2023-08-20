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
    }
    let mut aji = vec![vec![]; n + 1];
    for i in 0..n {
        input! {
            f_i: usize,
            s_i: usize
        }
        if aji[f_i-1].len() == 0 {
            aji[f_i-1].push(s_i);
        }
        else if aji[f_i-1].len() == 1 {
            let pre = aji[f_i-1][0];
            aji[f_i-1] = vec![max(pre, s_i), min(pre, s_i)];
        }
        else {
            let pre0 = aji[f_i-1][0];
            let pre1 = aji[f_i-1][1];
            if pre0 <= s_i {
                aji[f_i-1] = vec![s_i, pre0];
            }
            else if pre1 < s_i && s_i < pre0 {
                aji[f_i-1] = vec![pre0, s_i];
            }
        }
    }
    // println!("before aji={:?}", aji);
    aji.sort();
    aji.reverse();
    // println!("after aji={:?}", aji);

    let mut ans = 0;
    if aji[0].len() == 2 {
        ans = max(ans, aji[0][0] + aji[0][1] / 2);
    }
    if aji[1].len() >= 1 {
        ans = max(ans, aji[0][0] + aji[1][0]);
    }
    println!("{}", ans);

}