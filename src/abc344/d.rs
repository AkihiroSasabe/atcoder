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
    input! {
        t: Chars,
        n: usize,
    }
    let mut ss = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            si: [Chars; ai]
        }
        ss.push(si);
    }

    // dp[i][j] := 袋iまでで、T[j]まで一致しているときの最安値 
    let INF: usize = 1 << 60;
    let mut dp = vec![vec![INF; t.len()]; n];

    // dp[0] の初期化
    for i in 0..ss[0].len() {
        // i単語目
        if ss[0][i].len() > t.len() {continue}
        let is_ok = t[..ss[0][i].len()] == ss[0][i][..];
        if is_ok {
            dp[0][ss[0][i].len()-1] = 1;
        }
    }

    for i in 1..n {
        // i袋目

        // 前回の引き継ぎ
        dp[i] = dp[i-1].clone();
        
        // i袋目のj単語目
        for j in 0..ss[i].len() {

            // tの開始位置が0のとき
            if ss[i][j].len() > t.len() {continue}

            let is_ok = t[0..ss[i][j].len()] == ss[i][j][..];
            if is_ok {
                dp[i][ss[i][j].len()-1] = min(dp[i][ss[i][j].len()-1], 1);
            }

            // t の開始位置
            for k in 1..t.len() {
                if k+ss[i][j].len() > t.len() {continue}
                let is_ok = t[k..k+ss[i][j].len()] == ss[i][j][..];
                if is_ok {
                    dp[i][k-1 + ss[i][j].len()] = min(dp[i][k-1 + ss[i][j].len()], dp[i-1][k-1] + 1);
                }
            }
        }
    }

    if dp[n-1][t.len()-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dp[n-1][t.len()-1]);
    }


}