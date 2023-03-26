#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }
    let mut bb = b.clone();
    for yc in 0..r {
        for xc in 0..c {
            let num_c = b[yc][xc];
            if num_c != '#' && num_c != '.' {
                kernel(num_c, yc, xc, r, c, &mut bb);
            }
        }
    }
    for yc in 0..r {
        for xc in 0..c {
            print!("{}", bb[yc][xc]);
        }
        println!("");
    }

}

fn kernel(num_c: char, yc: usize, xc: usize, r:usize, c: usize, bb: &mut Vec<Vec<char>>) {
    let num: usize = num_c as usize - 48;
    let kernel = num * 2 + 1;

    for i in 0..kernel {
        for j in 0..kernel {
            let dist_y = i as isize - num as isize;
            let dist_x = j as isize - num as isize;
            if dist_y.abs() + dist_x.abs() > num as isize {continue}
            if yc + i < num {continue}
            if xc + j < num {continue}
            let y = yc + i - num;
            let x = xc + j - num;
            if y >= r || x >= c {continue}
            bb[y][x] = '.';
        }
    }

}