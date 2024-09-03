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
    // 尺取法
    let mut r = 2;
    let mut ans: usize = 0;
    let mut set = HashSet::new();
    for i in 0..n-1 {
        if set.contains(&i) {continue}
        set.insert(i);
        let mut count = 0;
        r = max(i+2, r);
        while r < n && (a[i+1] - a[i] == a[r] - a[r-1]) {
            set.insert(r-2);        
            r += 1;
            count += 1;
            ans += count;
        }
        // println!("i = {i}, ans = {:?}, r = {r}", ans);
    }

    ans += 2 * n - 1; // 自分と隣と、最後は自分だけ-1
    println!("{}", ans);
}