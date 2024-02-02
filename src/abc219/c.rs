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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2024-02-02 23:24-23:33 (9min)
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n]
    }
    let mut hash = HashMap::new();
    for i in 0..26 {
        hash.insert(x[i], i);
    }
    let mut ss = vec![];
    for i in 0..n {
        let mut si = vec![];
        for j in 0..s[i].len() {
            si.push(*hash.get(&s[i][j]).unwrap());
        }
        ss.push(si);
    }
    ss.sort();

    for i in 0..n {
        for j in 0..ss[i].len() {
            let num = ss[i][j];
            print!("{}", x[num]);
        }
        println!("");
    }

}