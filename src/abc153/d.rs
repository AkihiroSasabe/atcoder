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
        h: usize,
    }
    let mut ans = 0;
    // 
    // 2 -> (1,1) -> (0,1) -> (0,0): 3
    // 4 -> (2,2) -> 7
    // メモ化再帰
    let mut set = BTreeMap::new();
    let ans = dfs(h, &mut set);
    println!("{}", ans);

}

fn dfs(h: usize, set: &mut BTreeMap<usize, usize>) -> usize {
    if set.contains_key(&h) {
        return *set.get(&h).unwrap();
    }
    if h == 0 {
        return 0
    }
    else if h == 1 {
        return 1
    }
    else if h == 2 {
        return 3
    }
    let ans = 2 * dfs(h/2, set) + 1;
    set.insert(h, ans);
    return ans
}