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
    // 2025-04-27 0:56-01:24 (28min)
    input! {
        h: usize,
        w: usize,
        n: usize, // 電球
        m: usize, // ブロック
        ab: [(Usize1, Usize1);n], // 電球
        cd: [(Usize1, Usize1);m], // ブロック
    }

    // 壁
    let mut cols = vec![BTreeSet::new(); w];
    let mut rows = vec![BTreeSet::new(); h];

    // ブロックの座標
    for i in 0..m {
        let yi = cd[i].0;
        let xi = cd[i].1;
        cols[xi].insert(yi);
        rows[yi].insert(xi);
    }


    // いもす map
    let mut map_y = vec![vec![0; w]; h];
    let mut map_x = vec![vec![0; w]; h];

    for i in 0..n {
        let yi = ab[i].0;
        let xi = ab[i].1;
        if let Some(&block_y_max) = cols[xi].range(yi..).next() {
            map_y[block_y_max][xi] -= 1;
        }
        if let Some(&block_y_min) = cols[xi].range(..yi).rev().next() {
            map_y[block_y_min+1][xi] += 1;
        }
        else {
            map_y[0][xi] += 1;
        }

        if let Some(&block_x_max) = rows[yi].range(xi..).next() {
            map_x[yi][block_x_max] -= 1;
        }
        if let Some(&block_x_min) = rows[yi].range(..xi).rev().next() {
            map_x[yi][block_x_min+1] += 1;
        }
        else {
            map_x[yi][0] += 1;
        }
    }

    let mut map = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 1..w {
            map_x[y][x] = map_x[y][x-1] + map_x[y][x];
        }
        for x in 0..w {
            if map_x[y][x] > 0 {
                map[y][x] = true;
            }
        }
        // println!("map_x[{y}] = {:?}", map_x[y]);
    }

    for x in 0..w {
        for y in 1..h {
            map_y[y][x] = map_y[y-1][x] + map_y[y][x];
        }
        for y in 0..h {
            if map_y[y][x] > 0 {
                map[y][x] = true;
            }
        }
    }

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            if map[y][x] {
                ans += 1;
            }
        }
        // println!("map[{y}] = {:?}", map[y]);
    }
    println!("{}", ans);
}