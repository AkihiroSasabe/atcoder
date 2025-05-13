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
    let mut t = s.clone();

    let mut es = vec![];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == 'E' {
                es.push((y,x));
            }
        }
    }

    let inf: usize = 1<<60;
    let mut dists = vec![vec![inf; w]; h];
    let mut heap = BinaryHeap::new();

    for &(y, x) in es.iter() {
        dists[y][x] = 0;
        heap.push(Reverse((0, y, x)));
    }

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    // 左, 下, 右, 上
    let arrs = vec!['<', 'v', '>', '^'];
    

    while heap.len() > 0 {
        let Reverse((d, y, x)) = heap.pop().unwrap();

        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] == '#' {continue}
            if s[ny][nx] == 'E' {continue}
            if dists[ny][nx] != inf {continue}

            if dists[ny][nx] > d + 1 {
                dists[ny][nx] = d + 1;
                heap.push(Reverse((d+1, ny, nx)));
                t[ny][nx] = arrs[i];
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            print!("{}", t[y][x]);
        }
        println!("");
    }
}