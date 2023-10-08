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
    }
    let mut s = vec![];
    let mut c = vec![];
    let mut sc = vec![];
    for i in 0..n {
        input! {
            s_i: usize,
            c_i: usize,
        }
        s.push(s_i);
        c.push(c_i);
        sc.push(vec![s_i, c_i]);
    }

    // 小さいやつから貪欲にやるだけでは?
    sc.sort();
    let mut btree = BTreeMap::new();
    for i in 0..n {
        btree.insert(s[i], c[i]);
    }

    for i in 0..n {
        let size = sc[i][0];
        let mut count: usize = btree[&size];
        // println!("---- i = {i}, size = {size}, count = {count} ----");        

        let mut next_diff_count = 0;

        let mut next_size = size;
        while count > 1 {
            next_diff_count = count / 2;
            count = count % 2;
            *btree.get_mut(&next_size).unwrap() = count;
            next_size = 2 * next_size;
            *btree.entry(next_size).or_insert(0) += next_diff_count;
            count = btree[&next_size];
            // println!("next_size = {:?}, count = {}", next_size, count);
            // println!("btree = {:?}", btree);
        }
    }

    let mut ans = 0;
    for (size, count) in btree.iter() {
        ans += count;
    }
    println!("{}", ans);

}