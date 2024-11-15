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
        k: usize,
        s: Chars,
    }

    let mut i = 0;
    let mut ans = 0;
    loop {
        if i >= n {break}
        let mut is_ok = false;
        for j in 0..k {
            let ind = i + j;
            if ind >= n {break}
            if s[ind] == 'X' {break}
            if j == k-1 {
                is_ok = true;
            }
        }

        if is_ok {
            ans += 1;
            i += k;
        }
        else {
            i += 1;
        }
    }
    println!("{}", ans);

}