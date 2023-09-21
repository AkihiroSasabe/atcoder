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
        m: usize,
        s: Chars,
        c: [usize; n],
    }

    // 色毎に。
    let mut color_string = vec![vec![]; m];
    for i in 0..n {
        color_string[c[i] - 1].push(i);
    }
    let original_color_string = color_string.clone();
    for color in 0..m {
        // 最後の項を、先頭に持っていく。
        let last_pos = color_string[color].pop().unwrap();
        color_string[color].insert(0, last_pos);
    }
    let mut ans = vec!['a'; n];
    for color in 0..m {
        for i in 0..original_color_string[color].len() {
            let index = original_color_string[color][i];
            let new_index = color_string[color][i];
            ans[index] = s[new_index];
        }
    }
    for i in 0..n {
        print!("{}", ans[i]);
    }



}