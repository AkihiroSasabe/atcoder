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
    }
    let mut a = vec![];
    for i in 0..n {
        input! {
            ai: [usize; i+1]
        }
        a.push(ai);
    }
    for i in 0..n {
        for j in 0..i+1 {
            a[i][j] -= 1;
        }
    }

    let mut x = 0;
    for i in 0..n {
        let v_min = min(x, i);
        let v_max = max(x, i);
        x = a[v_max][v_min];
        // println!("{}", x+1);

    }
    println!("{}", x+1);

}