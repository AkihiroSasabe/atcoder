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
    // 2024-03-06 19:24-20:16 (52min)
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut aa = vec![vec![-1; w]; h];
    for y in 0..h {
        for x in 0..w {
            if a[y][x] == '+' {
                aa[y][x] = 1;
            }
        }
    }

    // 下か右にしか移動できない。
    // H,W <= 2_000
    // 4000!, 2000!

    // // dp[y][x] := マスが(y,x) にいる人が、今後得るであろう最大得点。
    // let mut dp: Vec<Vec<isize>> = vec![vec![0; w]; h];
    // // let mut dp1: Vec<Vec<isize>> = vec![vec![0; w]; h];

    // dp[h-1][w-2] = aa[h-1][w-1];
    // dp[h-2][w-1] = aa[h-1][w-1];
    // dp[h-2][w-2] = 

    // dp[y][x][0] := マスが(y,x) にいる人が、今後得るであろう最大得点。
    // dp[y][x][1] := マスが(y,x) にいない人が、今後得るであろう最大得点。
    let mut dp: Vec<Vec<Vec<isize>>> = vec![vec![vec![0; 2]; w]; h];
    let MIN_INF = - (1 << 60);
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            if y == h-1 && x == w-1 {continue}

            let (right0, right1) = if x + 1 < w {
                (
                    aa[y][x+1] + dp[y][x+1][1], 
                    dp[y][x+1][0]
                )
            } else {
                (MIN_INF, MIN_INF)
            };

            let (down0, down1) = if y + 1 < h {
                (
                    aa[y+1][x] + dp[y+1][x][1], 
                    dp[y+1][x][0]
                )
            } else {
                (MIN_INF, MIN_INF)
            };

            // dp[y][x][0] = max(right, right);

            if y == h - 1{
                dp[y][x][0] = right0;
                dp[y][x][1] = right1;
            }
            else if x == w - 1 {
                dp[y][x][0] = down0;
                dp[y][x][1] = down1;
            }
            else {
                if (right0 - right1) > (down0 - down1) {
                    dp[y][x][0] = right0;
                    dp[y][x][1] = right1;
                }
                else if (right0 - right1) < (down0 - down1) {
                    dp[y][x][0] = down0;
                    dp[y][x][1] = down1;
                }
                else {
                    dp[y][x][0] = right0;
                    dp[y][x][1] = right1;
                }
            }
        }
    }
    if dp[0][0][0] > dp[0][0][1] {
        println!("Takahashi");
    }
    else if dp[0][0][0] == dp[0][0][1] {
        println!("Draw");
    }
    else {
        println!("Aoki");
    }



}