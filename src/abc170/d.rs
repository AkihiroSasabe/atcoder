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
        mut a: [usize; n],
    }
    let mut counts = vec![0; 1_000_001];
    for i in 0..n {
        counts[a[i]] += 1;
    }

    let mut only_ones: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..1_000_001 {
        if counts[i] == 1 {
            only_ones.insert(i, 0);
        }
    }

    a.sort();
    a.dedup();
    let n = a.len();
    for i in 0..n {
        // 1個だけいたらいい。
        
        let mut count = 2;
        while count * a[i] <= 1_000_000 {
            let val = count * a[i];
            if only_ones.contains_key(&val) {
                *only_ones.get_mut(&val).unwrap() += 1;
            }
            count += 1;
        }
    }

    let mut ans= 0;
    for (ai, num) in only_ones {
        if num == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);



    
}