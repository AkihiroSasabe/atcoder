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
        n: usize
    }
    let mut keta = get_keta(n);
    let waru = 1;
    if keta <= 3 {
        println!("{}", n);
        return
    }
    let mut waru = 1;
    for _ in 0..keta-3 {
        waru *= 10;
    }
    let ans = n - n % waru;
    println!("{}", ans);
}

fn get_keta(mut x: usize) -> usize {
    let mut keta = 1;
    while x / 10 != 0 {
        x /= 10;
        keta += 1;
    }

    return keta
}