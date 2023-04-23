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
        S: Chars
    }

    let mut cond1 = false;
    let mut cond2 = false;

    let mut b_1 = 8;
    let mut b_2 = 8;
    for i in 0..8 {
        if b_1 == 8 && S[i] == 'B' {
            b_1 = i;
        }
        else if b_2 == 8 && S[i] == 'B' {
            b_2 = i;
        }
    }
    cond1 = (b_1 + b_2) % 2 == 1;
    let mut r_1 = 8;
    let mut r_2 = 8;
    let mut k_1 = 8;
    for i in 0..8 {
        if r_1 == 8 && S[i] == 'R' {
            r_1 = i;
        }
        else if r_2 == 8 && S[i] == 'R' {
            r_2 = i;
        }
        if S[i] == 'K' {
            k_1 = i;
        }
    }
    cond2 = r_1 < k_1 && k_1 < r_2;

    if cond1 && cond2 {
        println!("Yes");
    }
    else {
        println!("No");
    }

}