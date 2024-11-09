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
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..m {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai-1);
        b.push(bi-1);
    }

    let mut bans = BTreeSet::new();
    let mut dy = vec![2, 1, -1, -2, -2, -1, 1, 2];
    let mut dx = vec![1, 2, 2, 1, -1, -2, -2, -1];
    for i in 0..m {
        let ai = a[i];
        let bi = b[i];
        bans.insert((a[i], b[i]));

        for j in 0..8 {
            let ny = ai as isize + dy[j];
            let nx = bi as isize + dx[j];
            if ny < 0 || n as isize <= ny || nx < 0 || n as isize <= nx {continue}
            bans.insert((ny as usize, nx as usize));
        }
    }
    let ans = n * n - bans.len();
    println!("{}", ans);


}