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
        x:isize,
        n:usize,
        p:[isize; n],
    }

    let mut ys = vec![];
    for y in 0..1000 {
        let mut is_ok = true;
        for &pi in p.iter() {
            if y == pi {
                is_ok = false;
            }
        }
        if is_ok {
            ys.push(y);
        }
    }

    let mut abss = vec![];

    for y in ys {
        abss.push(((y-x).abs(), y));
    }
    abss.sort();
    println!("{}", abss[0].1);

}