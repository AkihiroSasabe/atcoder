#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    // n - m + (m-1) = n-1 
    for ys in 0..n-m+1 {
        // if ys + m - 1> n -1 {continue}
        for xs in 0..n-m+1 {
            // if xs + m - 1 > n -1 {continue}
            let mut is_ok = true;
            for dy in 0..m {
                for dx in 0..m {
                    if s[ys + dy][xs + dx] != t[dy][dx] {
                        is_ok = false;
                        break
                    }
                }
                if !is_ok {break}
            }
            if is_ok {
                println!("{} {}", ys + 1, xs + 1);
                return
            }
        }
    }
}