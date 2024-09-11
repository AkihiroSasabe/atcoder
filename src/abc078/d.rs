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
    // 2024-09-11 12:27-12:58 (31min, dpで解こうとしている)
    // 2024-09-11 19:45-19:58 (13min)
    // Total: 44 min
    input! {
        n: usize,
        z: isize, // これは実は使わない気がする
        w: isize,
        mut a: [isize; n],
    }

    // dp[i][0] := 今自分が i に居るとき、スコアの最大値
    // dp[i][1] := 今自分が i に居るとき、スコアの最小値
    let mut dp = vec![vec![0; 2]; n];
    let INF = 1 << 60;
    for i in 0..n {
        dp[i][1] = INF;
    }

    // 後ろから順に遷移を確定させていく
    for i in (0..n).rev() {
        // 相手のカード
        let opp = if i != 0 {
            a[i-1]
        } else {
            w
        };

        // 最後まで取ってゲームを終わらせる場合
        dp[i][0] = max(dp[i][0], (opp - a[n-1]).abs());
        dp[i][1] = min(dp[i][1], (opp - a[n-1]).abs());

        for j in i+1..n {
            // j は、相手の次のスタート位置
            dp[i][0] = max(dp[i][0], dp[j][1]);
            dp[i][1] = min(dp[i][0], dp[j][0]);
        }
    }
    // println!("dp = {:?}", dp);

    println!("{}", dp[0][0]);
}