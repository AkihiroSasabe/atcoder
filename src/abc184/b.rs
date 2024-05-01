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
        s: Chars,
    }
    let mut ans = x;
    for i in 0..n {
        if s[i] == 'o' {
            ans += 1;
        }
        else {
            if ans == 0 {continue}
            else {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}