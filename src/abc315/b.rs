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
        m: usize,
        d: [usize; m]
    }
    let mut total_days = 0;
    let mut mmdd = vec![];
    for i in 0..m {
        total_days+= d[i];
        for j in 0..d[i] {
            mmdd.push(vec![i + 1, j + 1]);
        }
    }
    let half = (total_days + 1) / 2 - 1;
    let mm = mmdd[half][0];
    let dd = mmdd[half][1];
    println!("{} {}", mm, dd);


    // 1 2 3
    // (3 + 1) / 2 - 1 = 2 - 1 = 1

}