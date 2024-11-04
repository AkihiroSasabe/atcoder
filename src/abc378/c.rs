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
    input! {
        n: usize,
        a: [isize; n],
    }
    // let INF = 1 << 60;
    // let mut b = vec![INF; n];

    print!("-1 ");
    let mut set = BTreeMap::new();
    set.insert(a[0], 1);
    for i in 1..n {
        if set.contains_key(&a[i]) {
            let ans = *set.get(&a[i]).unwrap();
            print!("{} ", ans);
        }
        else {
            print!("-1 ");
        }
        set.insert(a[i], i+1);
    }



}