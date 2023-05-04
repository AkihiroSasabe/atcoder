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
        a: [Chars; h],
        b: [Chars; h],
    }
    for dy in 0..h {
        for dx in 0..w {
            if check(&a, &b, dy, dx) {
                println!("Yes");
                return
            }
        }
    }
    println!("No");
}

fn check(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>, dy: usize, dx: usize) -> bool {
    let mut same_flag = true;
    for y in 0..a.len() {
        for x in 0..a[0].len() {
            if a[y][x] != b[(y + dy) % a.len()][(x + dx) % a[0].len()] {
                same_flag = false;
            }
        }
    }
    return same_flag
}

