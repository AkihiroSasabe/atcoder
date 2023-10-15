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
        n: Chars,
    }
    let mut ans = true;

    let mut pre = n[0];
    for i in 1..n.len() {
        if pre <= n[i] {
            ans = false;
            break
        }
        pre = n[i];
    }

    if ans {
        println!("Yes");
    }
    else {
        println!("No");

    }

}