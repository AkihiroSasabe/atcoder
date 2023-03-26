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
        a: [usize; n]
    }
    let mut hash = HashMap::new();

    for i in 0..n {
        if !hash.contains_key(&a[i]) {
            hash.insert(a[i], 1_usize);
        }
        else {
            *hash.get_mut(&a[i]).unwrap() += 1;
        }
    }

    let mut ans = 0;
    for (k, v) in hash.iter() {
        ans += (*v / 2);
    }
    println!("{}", ans);

}