#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, vec};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-07-09 12:51-13:00 (9min)
    // 2025-07-09 19:52-20:29 (37min)
    // Total: 46min
    input! {
        h: usize,
        w: usize,
        k: usize,
    }
    let mut r = vec![];
    let mut c = vec![];
    for i in 0..k {
        input!{
            ri: Usize1,
            ci: Usize1,
        }
        r.push(ri);
        c.push(ci);
    }

    // 多始点BFSの予感
    // let mut img = vec![vec![BTreeSet::nexw(); w]; h];
    // let mut img = vec![vec![HashSet::new(); w]; h];
    let mut img: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; w]; h];

    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut dists = vec![vec![init_distance; w]; h];

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    for v in 0..k  {
        let y = r[v];
        let x = c[v];
        dists[y][x] = 0;
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || img.len() as isize <= ny || nx < 0 || img[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            img[ny][nx][i] = true;
        }
    }



    let mut queue = VecDeque::new();
    for y in 0..h {
        for x in 0..w {
            let count = img[y][x].iter().filter(|&&b| b).count();
            if count >= 2 && dists[y][x] != 0 {
                dists[y][x] = 1;
                queue.push_back((y, x));
            }
        }
    }
    while queue.len() > 0 {
        let (y,x) = queue.pop_front().unwrap();

        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || img.len() as isize <= ny || nx < 0 || img[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            img[ny][nx][i] = true;
            let count = img[ny][nx].iter().filter(|&&b| b).count();
            if count >= 2 && dists[ny][nx] == init_distance {
                dists[ny][nx] = dists[y][x] + 1;
                queue.push_back((ny, nx));
            }
        }
    }

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            ans += if dists[y][x] == init_distance { 0 } else { dists[y][x] };
        }
    }
    println!("{}", ans);

}
