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
        k: usize,
        a: [usize; n],
    }
    
    let mut ans = (1 + k) * k / 2;
    let mut hash = HashSet::new();
    for i in 0..n {
        let ai = a[i];
        if ai <= k && !hash.contains(&ai) {
            ans -= ai;
            hash.insert(ai);
        }
    }
    println!("{}", ans);

    // 1,2,3,4,5 = 15

}