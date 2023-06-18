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
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut min_x = w;
    let mut max_x = 0;
    let mut min_y = h;
    let mut max_y = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                min_y = min(min_y, i);
                max_y = max(max_y, i);
                min_x = min(min_x, j);
                max_x = max(max_x, j);
            }
        }
    }

    let mut ans_y = min_y;
    let mut ans_x = min_x;
    for i in min_y..max_y+1 {
        for j in min_x..max_x+1 {
            if s[i][j] == '.' {
                ans_y = i + 1;
                ans_x = j + 1;
            }
        }
    }
    println!("{} {}", ans_y, ans_x);

}