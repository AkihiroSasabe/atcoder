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
        n: usize,
        q: usize,
        mut t: [usize; q],
    }
    for i in 0..q {
        t[i] -= 1;
    }
    let mut is_exist = vec![true; n];
    for i in 0..q {
        if is_exist[t[i]] {
            is_exist[t[i]] = false;
        }
        else {
            is_exist[t[i]] = true;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        if is_exist[i] {
            ans += 1;
        }
    }
    println!("{}", ans);


}