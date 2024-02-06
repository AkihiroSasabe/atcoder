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
        n: usize,
    }
    
    let dir_y = vec![-1,0,1,0];
    let dir_x = vec![0,1,0,-1];
    let mut d = 0;

    let mut s = vec![vec!['.'; w]; h];

    let mut y = 0;
    let mut x = 0;
    for i in 0..n {
        if s[y][x] == '.' {
            s[y][x] = '#';
            d += 1;
            d %= 4;
        }
        else {
            s[y][x] = '.';
            d += 3;
            d %= 4;
        }
        let dx = dir_x[d];
        let dy = dir_y[d];

        y = ((y + h) as isize + dy) as usize % h;
        x = ((x + w) as isize + dx) as usize % w;
    }
    for y in 0..h {
        for x in 0..w {
            print!("{}", s[y][x]);
        }
        println!();
    }

}