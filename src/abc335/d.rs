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
fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![0; n]; n];
    // ans[n/2][n/2] = 0;
    let mut seen = vec![vec![false; n]; n];

    for x in 0..n {
        ans[0][x] = x + 1;
        seen[0][x] = true;
    }
    for y in 0..n {
        ans[y][n-1] = n + y;
        seen[y][n-1] = true;
    }
    for x in 0..n {
        ans[n-1][x] = 3*n-2-x;
        seen[n-1][x] = true;
    }
    for y in 1..n {
        ans[y][0] = 4*n-3-y;
        seen[y][0] = true;
    }

    let yt = n/2;
    let xt = n/2;
    let mut ys = 1;
    let mut xs = 1;
    let mut num = ans[1][0] + 1;

    if n != 3 {
        loop {
            // 右
            (num, ys, xs) = toto(&mut ans, &mut seen, ys, xs, 0, num);
            ys += 1;
            if ys == yt && xs == xt {break}
    
            // 下
            (num, ys, xs) = toto(&mut ans, &mut seen, ys, xs, 1, num);
            xs -= 1;
            if ys == yt && xs == xt {break}
    
            // 左
            (num, ys, xs) = toto(&mut ans, &mut seen, ys, xs, 2, num);
            ys -= 1;
            if ys == yt && xs == xt {break}
    
            // 上
            (num, ys, xs) = toto(&mut ans, &mut seen, ys, xs, 3, num);
            xs += 1;
            if ys == yt && xs == xt {break}
        }
    }

    for y in 0..n {
        for x in 0..n {
            if y == yt && x == xt {
                print!("T ");
            }
            else {
                print!("{} ", ans[y][x]);
            }
        }
        println!("");
    }

}

fn toto(ans: &mut Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>, ys: usize, xs: usize, dir: usize, num: usize) -> (usize, usize, usize) {
    let n = ans.len();
    let dir_y = vec![0,1,0,-1];
    let dir_x = vec![1,0,-1,0];

    let mut count = 0;
    loop {
        let ny = (ys as isize + count * dir_y[dir]) as usize;
        let nx = (xs as isize + count * dir_x[dir]) as usize;
        if seen[ny][nx] {
            let y = (ys as isize + (count - 1) * dir_y[dir]) as usize;
            let x = (xs as isize + (count - 1) * dir_x[dir]) as usize;
            return (num + count as usize, y, x)
        }
        ans[ny][nx] = num + count as usize;
        seen[ny][nx] = true;
        count += 1;
    }

}