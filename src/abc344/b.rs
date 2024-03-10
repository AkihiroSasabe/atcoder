#![allow(dead_code, unused_imports)]
// use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
// use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // input! {
        
    // }
    let mut a = vec![];
    loop {
        let ai = read_usize();
        a.push(ai);
        if ai == 0 {
            break
        }
    }
    a.reverse();
    for x in a {
        println!("{}", x);
    }

}

// インタラクティブな読み込みをする関数 (1行に1個のusize)
fn read_usize() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}