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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut ans = vec![];
    for i in 0..(s.len()-1) {
        if s[i] == 'n' && s[i+1] == 'a' {
            ans.push('n');
            ans.push('y');
        }
        else {
            ans.push(s[i]);
        }
    }
    ans.push(s[s.len()-1]);
    for i in 0..ans.len() {
        print!("{}", ans[i]);
    }
}