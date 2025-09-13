#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    // 1番左上と右下
    let mut min_y = h;
    let mut min_x = w;
    let mut max_y = 0;
    let mut max_x = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                min_y = min(min_y, y);
                min_x = min(min_x, x);

                max_y = max(max_y, y);
                max_x = max(max_x, x);
            }
        }
    }

    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if s[y][x] == '.' {
                println!("No");
                return
            }
        }
    }
    println!("Yes");


}