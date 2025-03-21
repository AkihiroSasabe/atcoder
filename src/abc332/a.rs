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
        s: usize,
        k: usize,
    }
    let mut p = vec![];
    let mut q = vec![];
    for i in 0..n {
        input! {
            pi: usize,
            qi: usize,
        }
        p.push(pi);
        q.push(qi);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += p[i] * q[i];
    }
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}