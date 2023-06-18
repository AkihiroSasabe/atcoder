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
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
    }


    let mut dp = vec![vec![0; 2]; n];
    if x[0] == 1 {
        // 1: 毒
        dp[0][1] = max(dp[0][1], y[0]);
    }
    else {
        // 0: 健康
        dp[0][0] = max(dp[0][0], y[0]);
    }

    for i in 1..n {
        // 食べないケース
        // 毒 -> 毒
        dp[i][1] = max(dp[i][1], dp[i-1][1]);
        // 健康->健康
        dp[i][0] = max(dp[i][0], dp[i-1][0]);

        // 食べるケース
        if x[i] == 1 {
            // 元が健康でi番目を食べて毒になる
            // 健康 -> 毒
            dp[i][1] = max(dp[i][1], dp[i-1][0] + y[i]);
        }
        else {
            // 健康 -> 健康
            dp[i][0] = max(dp[i][0], dp[i-1][0] + y[i]);
            // 毒 -> 健康
            dp[i][0] = max(dp[i][0], dp[i-1][1] + y[i]);
        }
    }
    println!("{}", max(dp[n-1][0], dp[n-1][1]));

}