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
        d: [usize; n]
    }
    let mut cand = vec![vec![11, 1]; 9];
    for i in 0..9 {
        for j in 0..2 {
            cand[i][j] *= (i + 1);
        }
    }
    // println!("cand = {:?}", cand);

    let mut ans = 0;
    for i in 0..n {
        let m = i + 1;
        for j in 1..d[i]+1 {
            for k in 0..9 {
                if (m == cand[k][0] || m == cand[k][1]) && (j == cand[k][0] || j == cand[k][1]) {
                    ans += 1;
                    // println!("m, j = {:?}, {:?}", m, j);
                }
            }
            
        }
    }
    println!("{}", ans);

}