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
        s: Chars,
        t: Chars
    }

    let anum = 'A' as usize;

    let length = vec![0, 1, 2, 2, 1];
    let mut lens = vec![vec![]; 5];

    for i in 0..5 {
        for j in 0..5 {
            lens[i].push(length[(5 + j - i) % 5]);
        }
    }
    // println!("lens = {:?}", lens);
    let slen = lens[s[0] as usize - anum][s[1] as usize - anum];
    let tlen = lens[t[0] as usize - anum][t[1] as usize - anum];

    if tlen == slen {
        println!("Yes");
    }
    else {
        println!("No");
    }



}