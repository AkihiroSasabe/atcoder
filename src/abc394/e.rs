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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-24 13:07-14:16 (1h9min)
    input! {
        n: usize,
        c: [Chars; n]
    }
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let dp = bfs(&c);
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == init_distance {
                print!("-1 ");
            }
            else {
                    print!("{} ", dp[i][j]);
            }
        }
        println!("");
    }
}

fn bfs(c: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let n = c.len();

    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18

    // queueの要素 := (頂点uと頂点vの距離, 頂点u, 頂点v)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut dp = vec![vec![init_distance; n]; n];

    // 自身(0距離)の状態を格納
    for i in 0..n {
        dp[i][i] = 0;
        queue.push_back((0, i, i));
    }
    // 隣り合う辺(1距離)の状態を格納
    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            if c[i][j] == '-' {continue}
            dp[i][j] = 1;
            queue.push_back((1, i, j));
        }
    }
    // println!("queue = {:?}", queue);
    // let mut count = 20;
    while queue.len() > 0 {
        // println!("queue = {:?}", queue);
        let (length, u,v) = queue.pop_front().unwrap();

        // println!("(length, u, v) = {:?}", (length, u, v));
        // count -= 1;
        // if count == 0 {break}

        for nu in 0..n {
            for nv in 0..n {
                if c[nu][u] == '-' || c[v][nv] == '-' {continue}
                if c[nu][u] == c[v][nv] {
                    if dp[nu][nv] > dp[u][v] + 2 {
                        dp[nu][nv] = dp[u][v] + 2;
                        let new_state = (dp[nu][nv], nu, nv);
                        // println!("new_state = {:?}", new_state);
                        queue.push_back(new_state);
                    }
                }
            }
        }
    }
    return dp
}