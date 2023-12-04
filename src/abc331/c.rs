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
        a: [usize; n]
    }
    let mut a_sort = a.clone();
    a_sort.sort();

    let mut cum = vec![0; n];
    cum[0] = a_sort[0];
    for i in 1..n {
        cum[i] = cum[i-1] + a_sort[i];
    }
    for i in 0..n {
        let ind = a_sort.lower_bound(&(a[i] + 1));
        let ans = cum[n-1] - cum[ind - 1];
        print!("{} ", ans);
    }


}