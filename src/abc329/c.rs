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
        s: Chars
    }
    let a_num = 'a' as usize;
    let mut pre = s[0] as usize - a_num;
    let mut counts = vec![0; 26];
    let mut count = 1;
    counts[pre] = count;
    for i in 1..n {
        let now = s[i] as usize - a_num;
        if now == pre {
            count += 1;
        }
        else {
            count = 1;
        }
        counts[now] = max(counts[now], count);
        pre = now;
    }

    let mut ans = 0;
    for i in 0..26 {
        ans += counts[i];
    }
    // println!("counts = {:?}", counts);
    println!("{}", ans);
}