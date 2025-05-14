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

    let mut queue = VecDeque::new();
    queue.push_back(n);
    for i in (0..n).rev() {
        if s[i] == 'R' {
            // 一番左にiを追加
            queue.push_front(i);
        }
        else {
            queue.push_back(i);
        }
    }
    while let Some(ans) = queue.pop_front() {
        print!("{} ", ans);
    }
    

}