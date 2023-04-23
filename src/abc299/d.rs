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
fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let n = read();

    let mut ok = n;
    let mut ng = 1;
    while (ok as isize - ng as isize).abs() != 1 {
        let mid = (ok + ng) / 2;
        println!("? {}", mid);
        let s_i = read();
        if s_i == 1 {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("! {}", ng);

}