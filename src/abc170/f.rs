#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2025-06-27 20:46-21:30 (44min)
    // 2025-06-27 23:45-25:33 (48min)
    // 2025-06-28 12:55-13:10 (15min)
    // Total: 1h47min
    input! {
        h: usize,
        w: usize,
        k: usize,
        // スタート
        ys: Usize1,
        xs: Usize1,
        // ゴール
        yt: Usize1,
        xt: Usize1,
        c: [Chars; h]
    }

    // 頂点倍加?
    // 遅延セグ木?
    // dpやな。

    // v0が始点

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    #[cfg(debug_assertions)]
    let inf = 99;    
    
    #[cfg(not(debug_assertions))]
    let inf: usize = 1 << 60;

    // dp[y][x][d] = (最短距離, 最大余力)
    let mut dp = vec![vec![vec![(inf, Reverse(0)); 4]; w]; h];
    for d in 0..4 {
        dp[ys][xs][d].0 = 0;
    }
    let mut queue = VecDeque::new();
    // let mut heap = BinaryHeap::new();
    // 位置 
    let v0= (ys, xs);
    queue.push_back(v0);
    while queue.len() > 0 {
        // let v = heap.pop().unwrap();
        let v = queue.pop_front().unwrap();
        let (y, x) = v;
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || c.len() as isize <= ny || nx < 0 || c[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if c[ny][nx] == '@' {continue}

            // 現在の状態
            let (dist, Reverse(hp)) = dp[y][x][i];

            // 次の状態 (iと同じ方向)
            let (ndist, nhp) = if hp == 0 {(dist + 1, k-1)} else {(dist, hp-1)};
            if dp[ny][nx][i] > (ndist, Reverse(nhp)) {
                dp[ny][nx][i] = (ndist, Reverse(nhp));
                let nv = (ny, nx);
                // heap.push(nv);           
                queue.push_back(nv);           
            }

            // 次の状態 (iと異なる3方向)
            for diff in 1..4 {
                let nd = (i + diff) % 4;
                if dp[ny][nx][nd] > (ndist, Reverse(0)) {
                    dp[ny][nx][nd] = (ndist, Reverse(0));
                    // let nv = (Reverse(ndist), 0, ny, nx, nd);
                    let nv = (ny, nx);
                    // heap.push(nv);
                    queue.push_back(nv);           
                }
            }
        }
    }

    // debug
    #[cfg(debug_assertions)]
    {
    for y in 0..h {
        for x in 0..w {
            // print!("{:?} ", dp[y][x]);
            println!("{:?} ", dp[y][x]);
        }
        println!("");
    }
    }

    let mut ans = inf;
    for i in 0..4 {
        let cand = dp[yt][xt][i].0;
        ans = min(ans, cand);
    }
    if ans == inf {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}
