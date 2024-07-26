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
    // 2024-07-26 20:31-
    input! {
        d: usize,
        g: usize,
    }
    let mut p = vec![];
    let mut c = vec![];
    for i in 0..d {
        input!{
            pi: usize, // iの問題数 (100 x i  点)
            ci: usize, // iの全問回答ボーナス
        }
        p.push(pi);
        c.push(ci / 100);
    }
    let scale = 100;

    // 基本スコア: ユーザーが解いた問題すべての配点の合計です。

    // ナップサック問題
    // 問題数: d <= 10
    // 目標スコア: 100 <= g

    let INF: usize = 1 << 60;
    let mut dp = vec![INF; g / scale + 1];
    dp[0] = 0;
    for i in 0..d {
        let pre_dp = dp.clone();
        for src_point in 0..g / scale {
            for num in 1..p[i] {
                dp[min(src_point + num * (i + 1), g / scale)] = min(dp[min(src_point + num * (i + 1), g / scale)], pre_dp[src_point] + num);
            }
            // 最後はボーナス
            dp[min(src_point + p[i] * (i + 1) + c[i], g / scale)] = min(dp[min(src_point + p[i] * (i + 1) + c[i], g / scale)], pre_dp[src_point] + p[i]);
        }
    }
    println!("{}", dp[g / scale]);
    


}