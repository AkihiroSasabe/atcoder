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
    // 2025-03-01 13:19-13:26 (7min)
    input! {
        n: usize
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut cum: Vec<isize> = vec![0; 1_000_005];

    for i in 0..n {
        cum[a[i]] += 1;
        cum[b[i] + 1] -= 1;
    }
    let mut ans = cum[0];
    for i in 1..1_000_005 {
        cum[i] = cum[i] + cum[i-1];
        ans = max(cum[i], ans);
    }
    println!("{}", ans);
}