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
        x: usize,
        mut a: [usize; n-1],
    }
    // 
    a.sort();
    let mut sum = 0;
    for i in 1..(a.len()-1) {
        sum += a[i];
    }
    if sum + a[a.len()-1] < x {
        println!("-1");
        return;
    }
    if sum + a[0] >= x {
        println!("0");
        return;
    }
    if sum + a[0] < x {
        println!("{}", x - sum);
    }
    

}