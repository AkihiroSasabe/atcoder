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
use std::vec;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-05-15 21:08-21:32 (24min)
    input! {
        s: Chars, // 
        t: Chars, // 部分文字列
    }
    // s をhash で持っておいて。
    let mut set = HashSet::new();
    let mut map = HashMap::new();
    for i in 0..s.len() {
        set.insert(s[i]);
        map.entry(s[i]).or_insert(vec![]).push(i);
    }
    for i in 0..t.len() {
        if !set.contains(&t[i]) {
            println!("-1");
            return;
        }
    }
    // 2分探索でやれる。
    
    let mut ind = s.len() - 1; // 今いる場所
    let mut num_loop = -1;
    for i in 0..t.len() {
        let vector = map.get(&t[i]).unwrap();
        let nind = vector.lower_bound(&(ind+1));
        if nind == vector.len() {
            num_loop += 1;
            let nind = vector.lower_bound(&0);
            ind = vector[nind];
        }
        else {
            ind = vector[nind];
        }
    }

    let ans = 1 + ind + num_loop as usize * s.len();
    println!("{}", ans);



}