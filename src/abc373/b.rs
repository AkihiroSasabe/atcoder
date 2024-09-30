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
        s: Chars
    }
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut pos = vec![0; 26];
    for i in 0..s.len() {
        for j in 0..26 {
            if s[i] == uppercase[j] {
                pos[j] = i as isize;
                break
            }
        }
    }

    let mut ans = 0;
    for i in 1..26 {
        ans += (pos[i] - pos[i-1]).abs();
    } 
    println!("{}", ans);


}