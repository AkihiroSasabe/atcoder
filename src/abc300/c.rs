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
        c: [Chars; h],
    }
    // let mut c2 = vec![vec!['.'; w]; h];
    // for y in 0..h {
    //     for x in 0..w {
    //         c2[y+1][x+1] = c[y][x];
    //     }
    // }
    let mut s = vec![0; min(h, w) + 1];
    for cy in 1..(c.len()-1) {
        for cx in 1..(c[0].len()-1) {
            if is_center(&c, cy, cx) {
                s[get_n(&c, cy, cx)] += 1;
            }
        }
    }
    for i in 1..min(h, w) + 1 {
        print!("{} ", s[i]);
    }
}

fn is_center(c: &Vec<Vec<char>>, cy: usize, cx: usize) -> bool {
    if c[cy][cx] == '#' && c[cy-1][cx-1] == '#' && c[cy+1][cx+1] == '#' && c[cy-1][cx+1] == '#' && c[cy+1][cx-1] == '#' {
        return true
    }
    else {
        return false
    }
}

fn get_n(c: &Vec<Vec<char>>, cy: usize, cx: usize) -> usize {
    // let mut n = 1;
    let h: usize = c.len();
    let w: usize = c[0].len();
    for i in 2..min(h, w) {
        if cy < i || cx < i || h <= cy + i || w <= cx + i {
            return i - 1
        }
        if !(c[cy-i][cx-i] == '#' && c[cy+i][cx+i] == '#' && c[cy-i][cx+i] == '#' && c[cy+i][cx-i] == '#') {
            return i - 1
        }
    }
    return 1
}