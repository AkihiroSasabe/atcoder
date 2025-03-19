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
    solve_usize();
    // solve_f64();
}

fn solve_usize() {
    input! {
        x: Chars
    }
    let mut temp = 0;
    let a = x[0] as usize - '0' as usize;
    let b = x[1] as usize - '0' as usize;
    let c = x[3] as usize - '0' as usize;
    temp += a * 100 + b * 10 + c;
    let mut ans = 0;
    if temp >= 380 {
        ans = 1;
    }
    else if 375 <= temp && temp < 380 {
        ans = 2;
    } 
    else {
        ans = 3;
    }
    println!("{}", ans);
}

fn solve_f64() {
    input! {
        temp: f64
    }

    let mut ans = 0;
    if temp >= 38.0 {
        ans = 1;
    }
    else if 37.5 <= temp && temp < 38.0 {
        ans = 2;
    } 
    else {
        ans = 3;
    }
    println!("{}", ans);
}