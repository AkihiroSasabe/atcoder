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
        a: [usize; 4]
    }

    let mut nums = vec![0; 5];
    for i in 0..4 {
        nums[a[i]] += 1;
    }

    let mut ans = 0;
    for i in 0..5 {
        ans += nums[i] / 2;
    }
    println!("{}", ans);

    

}