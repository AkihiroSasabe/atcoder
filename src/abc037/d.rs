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
fn main() {
    // 2024-12-06 20:49-21:00 (11min)
    // 2024-12-07 10:33-10:34 (1min)
    // Total: 12min
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    // 10^6
    // メモ化再帰でいけそう。
    let mut dp = vec![vec![1; w]; h]; // (yi, xi) で始まって終わるパターンで+1
    let mut seen = vec![vec![false; w]; h];
    
    for y in 0..h {
        for x in 0..w {
            dfs(y, x, &a, &mut dp, &mut seen);
        }
    }

    let modulus = 1_000_000_007;
    let mut ans= 0;
    for y in 0..h {
        for x in 0..w {
            ans += dp[y][x];
            ans %= modulus;
        }
    }
    println!("{}", ans);

}

fn dfs(y: usize, x: usize, a: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>) {
    if seen[y][x] {return}
    seen[y][x] = true;
    let modulus = 1_000_000_007;

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    for i in 0..dir_y.len() {
        let ny = dir_y[i] + y as isize;
        let nx = dir_x[i] + x as isize;
        if ny < 0 || a.len() as isize <= ny || nx < 0 || a[0].len() as isize <= nx {continue}
        let ny = ny as usize;
        let nx = nx as usize;
        if a[ny][nx] <= a[y][x] {continue}
        dfs(ny, nx, a, dp, seen);
        dp[y][x] += dp[ny][nx];
        dp[y][x] %= modulus;
    }

}