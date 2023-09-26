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
        n: usize,
        m: usize,
        s: [Chars; n]
    }
    let mut seen = vec![vec![false; m]; n];
    // 右、下、左、上
    let dy = vec![0, 1, 0, -1];
    let dx = vec![1, 0, -1, 0];
    let mut checked = vec![vec![vec![false; 4]; m]; n];
    dfs(1, 1, &s, &mut seen, &mut checked, 0, &dy, &dx);
    let mut ans = 0;
    for y in 0..n {
        for x in 0..m {
            if seen[y][x] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn dfs(y: usize, x: usize, s: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, checked: &mut Vec<Vec<Vec<bool>>>, dir_index: usize, dy: &Vec<isize>, dx: &Vec<isize>) {
    seen[y][x] = true;
    checked[y][x][dir_index] = true;

    let next_y = dy[dir_index] + y as isize;
    let next_x = dx[dir_index] + x as isize;

    if next_y < 0 || next_y >= s.len() as isize || next_x < 0 || next_x >= s[0].len() as isize {
        return
    }
    let next_y = next_y as usize;
    let next_x = next_x as usize;

    if s[next_y][next_x] == '#' {
        // 岩とぶつかったら方向転換
        for i in 0..dy.len() {
            let next_y = (dy[i] + y as isize) as usize;
            let next_x = (dx[i] + x as isize) as usize;
            if checked[next_y][next_x][i] {continue}
            if s[next_y][next_x] == '#' {continue}
            dfs(next_y, next_x, s, seen, checked, i, dy, dx);
        }
    }
    else {
        dfs(next_y, next_x, s, seen, checked, dir_index, dy, dx);
    }
}