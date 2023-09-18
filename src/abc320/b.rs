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

    let mut ans = 1;
    for left in 0..s.len() {
        for right in left..s.len() {
            let mut is_kaibun = true;
            for i in left..right+1 {
                if s[i] != s[right-(i-left)] {
                    is_kaibun = false;
                    break
                }
            }
            if is_kaibun {
                ans = max(ans, 1 + right - left);
            }
        }
    }
    println!("{}", ans);
}