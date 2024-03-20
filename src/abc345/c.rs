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
        s: Chars
    }
    let n = s.len();
    let mut cum = vec![0; n + 1];
    for i in 1..n+1 {
        cum[i] = cum[i-1] + i;
    }

    let mut counts = vec![0; 26];
    for i in 0..n {
        let num = s[i] as usize - 'a' as usize;
        counts[num] += 1;
    }
    // println!("counts = {:?}", counts);

    let mut ans = cum[n-1];
    let mut num_two = 0;
    for i in 0..26 {
        if counts[i] > 1 {
            ans -= cum[counts[i]-1];
            num_two += 1;
        }
    }

    if num_two > 0 {
        ans += 1;
    }

    println!("{}", ans);
    
}
