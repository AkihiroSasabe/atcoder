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
        n: usize,
        d: usize,
        t: [usize; n]
    }

    let mut success_flag = false;
    let mut ans = 0;
    for i in 1..n {
        if t[i] - t[i-1] <= d {
            success_flag = true;
            ans = t[i];
            break
        }
    }
    if success_flag {
        println!("{}", ans);
    }
    else {
        println!("-1");
    }

}