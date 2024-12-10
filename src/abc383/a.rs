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
fn main() {
    input! {
        n: usize
    }
    let mut t = vec![];
    let mut v = vec![];
    let mut set = BTreeMap::new();
    for i in 0..n {
        input!{
            ti: usize,
            vi: usize,
        }
        t.push(ti);
        v.push(vi);
        set.insert(ti, (i, vi));
    }

    let mut ans = 0;
    for ti in 1..101 {
        if let Some((ind, vi)) = set.get(&ti) {
            ans += vi;
            if *ind == n-1 {
                println!("{}", ans);
            }
        }
        if ans > 0 {
            ans -= 1;
        }
    }


}