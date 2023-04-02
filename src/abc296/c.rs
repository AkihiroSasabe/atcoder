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
        x: isize,
        mut a: [isize; n]
    }

    let mut hash = HashMap::new();

    for i in 0..n {
        hash.insert(a[i], 0);
    }
    a.sort();
    let mut flag = false;
    for i in 0..n {
        if hash.contains_key(&(a[i] + x)) {
            flag = true;
            break
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


}