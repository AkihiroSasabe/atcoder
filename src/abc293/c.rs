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
        a: [[usize; w]; h]
    }

    let mut ans = 0;
    let mut seen: HashMap<usize, usize> = HashMap::new();
    dfs(&a, 0, 0, h-1, w-1, &mut ans, &mut seen);
    println!("{}", ans);

}

fn dfs(a: &Vec<Vec<usize>>, y: usize, x: usize, gy: usize, gx: usize, ans: &mut usize, seen: &mut HashMap<usize, usize>) {
    // hash.insert(x + gx*y, 0);
    seen.insert(a[y][x], 0);
    // println!("x, y = {} {} ", x, y);
    if x == gx && y == gy {
        *ans += 1;
        seen.remove(&a[y][x]);
        return;
    }
    for i in 0..2 {
        // down
        let mut nx = x;
        let mut ny = y;
        if i == 0 {
            ny += 1;
        }
        if i == 1 {
            nx += 1;
        }
        if nx > gx || ny > gy {continue}
        if seen.contains_key(&a[ny][nx]) {continue}
        // if hash.contains_key(nx + gx*ny) {continue}
        dfs(a, ny, nx, gy, gx, ans, seen);
    }
    seen.remove(&a[y][x]);

}