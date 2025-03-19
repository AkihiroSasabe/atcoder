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
        mut s: Chars
    }

    let mut ans = 0;
    let mut is_i = true;
    for i in 0..s.len() {
        if is_i {
            if s[i] == 'o' {
                ans += 1;
                is_i = !is_i;
            }
        }
        else {
            if s[i] == 'i' {
                ans += 1;
                is_i = !is_i;
            }
        }
        is_i = !is_i;
    }
    if !is_i {
        ans += 1;
    } 
    println!("{}", ans);


}