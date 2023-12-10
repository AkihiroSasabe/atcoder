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
    // 2023-12-09 23:46-
    input! {
        n: usize,
    }

    // 
    // 5
    // 101 <- +1, *2, *2, +1
    // 13
    // 1101 <- +1, *2, +1, *2, +1

    // 1    +1
    // 10   *2
    // 11   +1
    // 110  *2
    // 1100 *2
    // 1101 +1

    // 101
    // 1 +1
    // 10 *2
    // 100 *2
    // 101 +1

    // 1152921504606846976
    // 1000000000000000000

    let mut ans = vec![];
    let mut flag = false;
    for i in (0..61).rev() {
        if 1 << i & n != 0 {
            ans.push('A');
            flag = true;
        }
        if flag {
            if i != 0 {
                ans.push('B');
            }
        }
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
    }




}