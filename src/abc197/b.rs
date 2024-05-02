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
        h: usize,
        w: usize,
        y: usize,
        x: usize,
        s: [Chars; h],
    }
    let mut ans = 1;

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    for i in 0..dir_y.len() {
        let mut x0 = x-1;
        let mut y0 = y-1;
        loop {
            let ny = dir_y[i] + y0 as isize;
            let nx = dir_x[i] + x0 as isize;
            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {break}
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] == '#' {break}
            ans += 1;
            y0 = ny;
            x0 = nx;
        }
    }
    println!("{}", ans);


}