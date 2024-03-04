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
        a: [[usize; n]; n]
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                graph[i].push(j);
            }
        }
    }

    for i in 0..n {
        for v in graph[i].iter() {
            print!("{} ", v + 1);
        }
        println!("");
    }
}