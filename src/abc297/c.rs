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
        mut s: [Chars; h]
    }
    // let mut ans = s.clone();
    for y in 0..h {
        for x in 1..w {
            if s[y][x-1] == 'T' && s[y][x] == 'T' {
                s[y][x-1] = 'P';
                s[y][x] = 'C';
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            print!("{}", s[y][x]);
        }
        println!("");
    }

}