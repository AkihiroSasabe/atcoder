use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut g = vec![];
    for i in 0..h {
        input! {
            g_i: Chars
        }
        g.push(g_i);
    }

    let mut seen = vec![vec![false; w]; h];
    let ans = dfs(&g, &mut seen, 0, 0, h, w);

    let INF = 1 << 60;
    if ans[0] == INF{
        println!("-1");
    }
    else {
        println!("{} {}", ans[0] + 1, ans[1] + 1);
    }
    
}

fn dfs(g: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, mut y: usize, mut x: usize, h: usize, w: usize) -> Vec<usize> {
    seen[y][x] = true;
    let INF = 1 << 60;

    if g[y][x] == 'U' {
        // 移動不可
        if y == 0 {
            let ans = vec![y, x];
            return ans;
        }
        else {
            // ループしてるとき
            if seen[y-1][x] {
                let ans = vec![INF, INF];
                return ans;
            }
            // 移動可能なとき
            else {
                let ans = dfs(g, seen, y-1, x, h, w);
                return ans;
            }
        }
    }
    else if g[y][x] == 'D' {
        // 移動不可
        if y == h - 1 {
            let ans = vec![y, x];
            return ans;
        }
        else {
            // ループしてるとき
            if seen[y+1][x] {
                let ans = vec![INF, INF];
                return ans;
            }
            // 移動可能なとき
            else {
                let ans = dfs(g, seen, y+1, x, h, w);
                return ans;
            }
        }
    }
    else if g[y][x] == 'L' {
        // 移動不可
        if x == 0 {
            let ans = vec![y, x];
            return ans;
        }
        else {
            // ループしてるとき
            if seen[y][x-1] {
                let ans = vec![INF, INF];
                return ans;
            }
            // 移動可能なとき
            else {
                let ans = dfs(g, seen, y, x-1, h, w);
                return ans;
            }
        }
    }
    else {
        // 移動不可
        if x == w-1 {
            let ans = vec![y, x];
            return ans;
        }
        else {
            // ループしてるとき
            if seen[y][x+1] {
                let ans = vec![INF, INF];
                return ans;
            }
            // 移動可能なとき
            else {
                let ans = dfs(g, seen, y, x+1, h, w);
                return ans;
            }
        }
    }
}