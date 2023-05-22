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
        m: usize,
        d: isize,
        mut a: [isize; n],
        mut b: [isize; m]
    }
    a.sort();
    b.sort();
    a.reverse();
    b.reverse();
    // println!("a={:?}", a);
    // println!("b={:?}", b);
    let mut ans = -1;
    let mut b_ind_max = 0;
    for i in 0..a.len() {
        while a[i] + d < b[b_ind_max] {
            if b_ind_max == m - 1 {break}
            b_ind_max += 1;
        }
        if a[i] - d <= b[b_ind_max] && b[b_ind_max] <= a[i] + d{
            // println!("ans={}, a[i]={}, b[b_ind_max]={}, sum={}", ans, a[i], b[b_ind_max], a[i] + b[b_ind_max]);
            ans = max(ans, a[i] + b[b_ind_max]);
        }
    }
    println!("{}", ans);


}