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
        s: Chars
    }

    let mut ans = vec![];
    let mut count = 0;

    for i in 1..s.len() {
        if s[i] == '-' {
            count += 1;
        }
        else {
            ans.push(count);
            count = 0;
        }
    }
    for i in ans {
        print!("{} ", i);
    }

}