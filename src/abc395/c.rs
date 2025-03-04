#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let inf = 1 << 62;
    // let mut pos = vec![inf; 1_000_001];
    let mut btree = BTreeMap::new();

    let mut ans = inf;
    for i in 0..n {
        if let Some(&pos) = btree.get(&a[i]) {
            ans = min(ans, 1 + i - pos);
        }
        btree.insert(a[i], i);
    }
    if ans == inf {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }


}