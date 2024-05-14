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
        mut a: [usize; n]
    }
    a.sort();
    // let mut sum = 0;
    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * (n-1);
    }
    // ans /= 2;
    
    // 2分探索
    let mut sub = 0;
    let x = 100_000_000;
    for i in 0..n {
        let diff = x - a[i];
        let ind = a.lower_bound(&diff);
        if ind <= i {
            sub += n - (i + 1);
        }
        else {
            sub += n - ind;
        }
    }
    // println!("sub = {:?}", sub);
    // sub /= 2;
    ans -= sub * x;
    println!("{}", ans);


}