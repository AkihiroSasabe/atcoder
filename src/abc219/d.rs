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
    // 2023-01-22 18:22-19:39 (1h17m)
    // 22:56-0:03 (1h7m)
    // 2h24m
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
    }
    let INF: usize = 1 << 60;
    // dp[n][x][y] := x個以上のたこ焼き、y個以上のたい焼きを買うのに必要な最小の弁当数
    let mut dp = vec![vec![vec![INF; 301]; 301]; n];

    // 初期化
    for i in 0..n {
        dp[i][0][0] = 0;
    }
    for xx in 0..a[0]+1 {
        for yy in 0..b[0]+1 {
            if xx == 0 && yy == 0 {continue}
            dp[0][xx][yy] = 1;
        }
    }

    for i in 1..n {
        for xx in 0..x+1 {
            for yy in 0..y+1 {
                dp[i][xx][yy] = min(dp[i][xx][yy], dp[i-1][xx][yy]);
                let mut next_xx = xx + a[i];
                let mut next_yy = yy + b[i];
                if next_xx > x {next_xx = x}
                if next_yy > y {next_yy = y}
                dp[i][next_xx][next_yy] = min(dp[i][next_xx][next_yy], dp[i-1][xx][yy] + 1);
            }
        }
        for xx_temp in 0..x+1 {
            let xx2 = x - xx_temp;
            if xx2+1 <= x {
                dp[i][xx2][y] = min(dp[i][xx2][y], dp[i][xx2+1][y]);
            }
            for yy_temp in 0..y+1 {
                let yy2 = y - yy_temp;
                if yy2 + 1 <= y {
                    dp[i][xx2][yy2] = min(dp[i][xx2][yy2], dp[i][xx2][yy2+1]);
                } 
            }
        }
    }

    if dp[n-1][x][y] != INF {
        println!("{}", dp[n-1][x][y]);
    }
    else {
        println!("{}", -1);
    }

}