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
    // 2024-08-10 21:02-21:32 (30min)
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut m = 0;
    // let mut last_ind = 0;
    for i in (0..n).rev() {
        if m <= s[i].len() {
            // m = max(s[i].len(), m);
            m = s[i].len();
            // last_ind = i;
        }
    }

    let mut last_inds = vec![0; m+1];


    for mi in 1..m+1 {
        for i in 0..n {
            if s[i].len() >= mi {
                last_inds[mi] = n - 1 - i;
                break
            }
        }
    }

    let mut t = vec![vec![' '; n]; m];

    for i in (0..n).rev() {
        let tx = n - 1 - i;
        for j in 0..m {
            let ty = j;

            if j < s[i].len() {
                t[ty][tx] = s[i][j];
            }
            else {
                if tx <= last_inds[ty + 1] {
                    t[ty][tx] = '*';
                } 
                else {
                    t[ty][tx] = ' ';
                }
            }
        }
    }

    for ty in 0..m {
        for tx in 0..n {
            if t[ty][tx] != ' ' {
                print!("{}", t[ty][tx]);
            }
        }
        println!("");
    }
}