#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                let mut count = 0;
                for i in 0..dir_y.len() {
                    let ny = dir_y[i] + y as isize;
                    let nx = dir_x[i] + x as isize;
                    if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
                    let ny = ny as usize;
                    let nx = nx as usize;
                    if s[ny][nx] == '#' {
                        count += 1;
                    }
                }
                if !(count == 2 || count == 4) {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");

}