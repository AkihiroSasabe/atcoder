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
    }
    let mut j_cand = vec![];
    let mut n_over_j = vec![];
    for j in 1..10 {
        if n % j == 0 {
            j_cand.push(j);
            n_over_j.push(n / j);
        }
    }
    let mut s = vec![1];
    for i in 1..n+1 {
        let mut jj = 0;
        for x in 0..j_cand.len() {
            if i % n_over_j[x] == 0 {
                jj = j_cand[x];
                break;
            }
        }
        s.push(jj);
    }
    
    for i in 0..s.len() {
        if s[i] != 0 {
            print!("{}", s[i]);
        }
        else {
            print!("-");
        }
    }



}