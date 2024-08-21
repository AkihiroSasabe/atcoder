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
    // 2024-08-20 19:44-20:34 (50min)
    input! {
        n: usize,
        k: usize,
        mut x: [usize; n],
        a: [usize; n],
    }
    x.iter_mut().for_each(|xi: &mut usize| *xi -= 1);
    let mut xs: Vec<Vec<usize>> = vec![vec![0; n]; 63];
    xs[0] = x.clone();
    for i in 1..63 {
        for j in 0..n {
            xs[i][j] = xs[i-1][xs[i-1][j]];
        }
    }

    let mut inds: Vec<usize> = (0..n).collect();
    for i in 0..63 {
        if k & (1 << i) != 0 {
            // println!("i = {:?}", i);
            let pre_inds: Vec<usize>= inds.clone();
            for j in 0..n {
                inds[j] = pre_inds[xs[i][j]];
            }
        }
    }
    // println!("inds = {:?}", inds);

    for i in 0..n {
        print!("{} ", a[inds[i]]);
    }
}


