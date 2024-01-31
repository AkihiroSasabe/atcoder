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
        q: [isize; n],
        a: [isize; n],
        b: [isize; n],
    }

    let mut ans = 0;
    for anum in 0..1_000_001 {
        let mut remain = vec![0; n];
        let mut flag = true;
        for j in 0..n {
            remain[j] = q[j] - anum * a[j];
            if remain[j] < 0 {
                flag = false;
                break;
            } 
        }
        if !flag {break}

        let mut bnum = 1_000_001;
        for j in 0..n {
            if b[j] == 0 {continue}
            let cand = remain[j] / b[j];
            bnum = min(bnum, cand);
        }
        ans = max(ans, anum + bnum);
    }
    println!("{}", ans);
}