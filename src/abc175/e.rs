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
    // 2024-04-06 11:10-11:47 (37min)
    input! {
        r: usize,
        c: usize,
        k: usize,
    }
    let mut rr = vec![];
    let mut cc = vec![];
    let mut v = vec![];
    let mut img = vec![vec![0; c]; r];
    for i in 0..k {
        input!{
            ri: usize,
            ci: usize,
            vi: usize,
        }
        rr.push(ri-1);
        cc.push(ci-1);
        v.push(vi);
        img[ri-1][ci-1] = vi;
    }

    // 同じ行は3個まで拾える
    // 貰うdpでok
    let mut dp = vec![vec![vec![0; 4]; c]; r];
    dp[0][0][1] = img[0][0];
    for y in 0..r {
        for x in 0..c {
            for i in 0..4 {
                if y > 0 {
                    // 上から遷移
                    // y,xを拾わない
                    dp[y][x][0] = max(dp[y][x][0], dp[y-1][x][i]);

                    // y,xを拾う
                    dp[y][x][1] = max(dp[y][x][1], dp[y-1][x][i] + img[y][x]);
                }
                if x > 0 {
                    // 左から遷移
                    // y,xを拾わない
                    dp[y][x][i] = max(dp[y][x][i], dp[y][x-1][i]);
                }
            }
            for i in 0..3 {
                if x > 0 {
                    // y,xを拾う
                    dp[y][x][i+1] = max(dp[y][x][i+1], dp[y][x-1][i] + img[y][x]);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..4 {
        ans = max(ans, dp[r-1][c-1][i]);
    }
    println!("{}", ans);


}