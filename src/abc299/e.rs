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
    input! {
        n: usize,
        m: usize
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(u_i-1);
        graph[v_i-1].push(v_i-1);
    }
    for k in 0..k {
        
    }
}