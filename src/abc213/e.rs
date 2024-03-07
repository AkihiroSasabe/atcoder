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
    // 2024-02-06 20:05-21:00 (55min)
    // 2024-02-07 19:37-20:10 (33min)
    // 2024-02-07 20:10-20:43 (33min) 解説見て実装 いわゆる、01-BF
    // total 121min
    // 01BFS: https://betrue12.hateblo.jp/entry/2018/12/08/000020
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    // 全探索 500 * 500 = 250_000 = 2.5 * 10^5
    let mut dp = vec![vec![vec![0; 2]; w]; h];

    // 右、下、左、上
    let mut dir_y = [0, 1, 0, -1];
    let mut dir_x = [1, 0, -1, 0];

    let mut deque = VecDeque::new();
    let init_dist = h * w;
    let mut dist = vec![vec![init_dist; w]; h];
    dist[0][0] = 0;
    deque.push_back((0, 0));

    while deque.len() != 0 {
        let (y, x) = deque.pop_front().unwrap();

        for d in 0..4 {
            let dy = dir_y[d];
            let dx = dir_x[d];

            let ny = y + dy;
            let nx = x + dx;

            if nx < 0 || nx >= w as isize {continue}
            if ny < 0 || ny >= h as isize {continue}
            let y = y as usize;
            let x = x as usize;
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] == '#' {
                // 隣が壁
                // ny, nx の壁が破壊されるような殴り方は、4通り
                // このとき、ny,nxの周囲9マスは、1回のパンチで通れるようになる。
                for dny in -1..2 {
                    for dnx in -1..2 {
                        let nny = dny + ny as isize;
                        let nnx = dnx + nx as isize;
                        if nnx < 0 || nnx >= w as isize {continue}
                        if nny < 0 || nny >= h as isize {continue}
                        // <=> 重み1のエッジなので、末尾に突っ込む
                        if dist[nny as usize][nnx as usize] > dist[y][x] + 1{
                            dist[nny as usize][nnx as usize] = dist[y][x] + 1;
                            deque.push_back((nny as isize, nnx as isize));
                        }
                        else {
                            continue
                        }
                    }
                }
            }
            else {
                // 壁じゃない 
                // <=> 重み0のエッジなので、先頭に突っ込む
                if dist[ny][nx] > dist[y][x] {
                    dist[ny][nx] = dist[y][x];
                    deque.push_front((ny as isize, nx as isize));
                }
                else {
                    continue
                }
            }
        }
    }
    // println!("dist = {:?}", dist);
    let ans = dist[h-1][w-1];
    println!("{}", ans);


}