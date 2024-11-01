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
    // 2024-10-31 20:42-21:27 (45min)
    // 2024-11-01 12:28-12:38 (10min)
    // 55min
    input! {
        n: usize,
        s: Chars
    }

    // dpでいけそうな雰囲気
    // 2: 未, 0: 羊, 1: 狼

    // dp[0][0][i] := 先頭が羊で、2番目も羊のときの、i番目の動物
    // dp[0][1][i] := 先頭が羊で、2番目が狼のときの、i番目の動物
    // dp[1][0][i] := 先頭が狼で、2番目が羊のときの、i番目の動物
    // dp[1][1][i] := 先頭が狼で、2番目も狼のときの、i番目の動物
    let mut dp = vec![vec![vec![2; n]; 2]; 2];
    
    dp[0][0][0] = 0;
    dp[0][0][1] = 0;

    dp[0][1][0] = 0;
    dp[0][1][1] = 1;

    dp[1][0][0] = 1;
    dp[1][0][1] = 0;

    dp[1][1][0] = 1;
    dp[1][1][1] = 1;


    if s[0] == 'o' {
        dp[0][0][n-1] = 0;
        dp[0][1][n-1] = 1;

        dp[1][0][n-1] = 1;
        dp[1][1][n-1] = 0;
    }
    else {
        dp[0][0][n-1] = 1;
        dp[0][1][n-1] = 0;

        dp[1][0][n-1] = 0;
        dp[1][1][n-1] = 1;
    }


    let mut ng_xy = BTreeSet::new();
    for i in 1..n {
        for x in 0..2 {
            for y in 0..2 {
                if i == n-2 || i == n-1 {
                    // 最後だけチェックが入る
                    let mut is_ok = true;
                    if s[i] == 'o' && dp[x][y][i] == 0 {
                        if dp[x][y][(i+1) % n] != dp[x][y][i-1] {
                            is_ok = false;
                        }
                    }
                    else if s[i] == 'x' && dp[x][y][i] == 0 {
                        if dp[x][y][(i+1) % n] != 1 - dp[x][y][i-1] {
                            // ok_xy.push((x, y));
                            is_ok = false;
                        }
                    }
                    else if s[i] == 'o' && dp[x][y][i] == 1 {
                        if dp[x][y][(i+1) % n] != 1 - dp[x][y][i-1] {
                            is_ok = false;
                            // ok_xy.push((x, y));
                        }
                    }
                    else if s[i] == 'x' && dp[x][y][i] == 1 {
                        if dp[x][y][(i+1) % n] != dp[x][y][i-1] {
                            is_ok = false;
                            // ok_xy.push((x, y));
                        }
                    }
                    if !is_ok {
                        ng_xy.insert((x, y));
                    }
                }
                else {
                    if s[i] == 'o' && dp[x][y][i] == 0 {
                        dp[x][y][i+1] = dp[x][y][i-1];
                    }
                    else if s[i] == 'x' && dp[x][y][i] == 0 {
                        dp[x][y][i+1] = 1 - dp[x][y][i-1];
                    }
                    else if s[i] == 'o' && dp[x][y][i] == 1 {
                        dp[x][y][i+1] = 1 - dp[x][y][i-1];
                    }
                    else if s[i] == 'x' && dp[x][y][i] == 1 {
                        dp[x][y][i+1] = dp[x][y][i-1];
                    }
                }
            }
        }
    }


    for x in 0..2 {
        for y in 0..2 {
            if !ng_xy.contains(&(x, y)) {
                for i in 0..n {
                    if dp[x][y][i] == 0 {
                        print!("S");
                    }
                    else {
                        print!("W");
                    }
                }
                println!("");
                return
            }
        }
    }
    println!("-1");


}

