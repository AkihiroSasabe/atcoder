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
    // let mut hash = HashMap::new();
    let mut count = vec![0; 26];
    for i in 0..s.len() {
        let num = s[i] as usize - 'a' as usize;
        count[num] += 1;
    }
    let mut c2 = vec![0; 1000];
    for i in 0..26 {
        c2[count[i]] += 1;
    }
    // let mut flag = true;
    for i in 1..1000 {
        if !(c2[i] == 0 || c2[i] == 2) {
            // flag = false;
            println!("No");
            return
        }
    }
    println!("Yes");

}