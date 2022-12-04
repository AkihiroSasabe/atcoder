#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}