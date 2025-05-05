#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        s: Chars
    }
    let mut lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut is_used = vec![false; 26];
    for si in s {
        for i in 0..26 {
            if si == lowercase[i] {
                is_used[i] = true;
            }
        }
    }
    for i in 0..26 {
        if !is_used[i] {
            println!("{}", lowercase[i]);
            return;
        }
    }




    
}