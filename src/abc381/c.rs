#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
        n: usize,
        s: Chars
    }
    // 連続する1の長さと、2の長さを求めればいいんじゃね?
    let mut ans = 1;

    let mut len_1 = vec![0; n];
    let mut len_2 = vec![0; n];
    if s[0] == '1' {
        len_1[0] = 1;
    }
    if s[n-1] == '2' {
        len_2[n-1] = 1;
    }

    for i in 1..n {
        if s[i] == '1' {
            len_1[i] = len_1[i-1] + 1;
        }
    }
    for i in (0..n-1).rev() {
        if s[i] == '2' {
            len_2[i] = len_2[i+1] + 1;
        }
    }

    for i in 1..n-1 {
        if s[i] == '/' {
            ans = max(ans, 1 + 2 * min(len_1[i-1], len_2[i+1]) );
        }
    }

    println!("{}", ans);



}