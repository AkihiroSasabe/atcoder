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
use rand::Rng;
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    // 7_873_200
    // 17_714_700
    let mut seen = vec![vec![false; w]; h];
    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {continue}
            dfs(0, k, &mut ans, y, x, &s, &mut seen);
        }
    }
    println!("{}", ans);

}

fn dfs(d: usize, k: usize, ans: &mut usize, y: usize, x: usize, s: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) {
    if d == k {
        *ans += 1;
        return 
    }

    seen[y][x] = true;

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    for i in 0..dir_y.len() {
        let ny = dir_y[i] + y as isize;
        let nx = dir_x[i] + x as isize;
        if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
        let ny = ny as usize;
        let nx = nx as usize;
        if seen[ny][nx] {continue}
        if s[ny][nx] == '#' {continue}
        dfs(d + 1, k, ans, ny, nx, s, seen);
    }

    seen[y][x] = false;
}