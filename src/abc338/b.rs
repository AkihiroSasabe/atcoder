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

    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();


    let mut counts = vec![0; 26];

    for i in 0..s.len() {
        let ind = s[i] as usize - 'a' as usize;
        counts[ind] += 1;
    }

    let mut ans = vec![0, 0];
    for i in 0..26 {
        if counts[i] > ans[0] {
            ans[0] = counts[i];
            ans[1] = i;
        }
    }
    println!("{}", lowercase[ans[1]]);
}