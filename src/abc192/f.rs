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
    // 2025-06-11 19:18-20:10 (52min)
    // 【N<=100なら3次元DP、a[i]を選ばないときの遷移を忘れないように！】
    // 長さN（<=100）の数列からk個の項を選んで和Sを取る。目標値をXとしたとき、X = S + k T となるような最小のTを求める問題。
    // X-Sがkの倍数にならないといけないので、Kの剰余についてDPをするとよい。i番目の要素を選ばないときの遷移を書き忘れてデバッグに時間が掛かった。
    // dp[i][k0][k1] := i番目までで、k0個の要素を選んでいて、和の剰余がk1になっている最大値。
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.sort();


    // 1種類のとき
    // 2種類のとき
    // ...
    // 3種類のとき

    // k種類のポーションの和が、x以下で最大になるものがあるかを選択。


    
    let inf = 10_000_000_000_000_000_000;
    let mut ans = inf;
    for k in 1..n+1 {
        // println!("k = {k} ----------");
        // 差分がk で割れないといけない。
        // x % k == sum % k にならないといけない。
        // n個の中からk個
        // dp[i][k0][k1] := i番目までで、k0個の要素を選んでいて、和の剰余がk1になっている最大値。
        let mut dp = vec![vec![vec![inf; k]; k+1]; n];
        dp[0][0][0] = 0;
        dp[0][1][a[0] % k] = a[0];
        for i in 1..n {
            for k0 in 0..k {
                for k1 in 0..k {
                    // println!("dp[{}][{}][{}] = {:?}", i-1,k0,k1, dp[i][k0][k1]);
                    if dp[i-1][k0][k1] == inf {continue}

                    if dp[i][k0][k1] == inf {
                        dp[i][k0][k1] = dp[i-1][k0][k1];
                    }
                    else {
                        dp[i][k0][k1] = max(dp[i][k0][k1], dp[i-1][k0][k1]);
                    }
                    let r = (dp[i-1][k0][k1] + a[i]) % k;

                    if dp[i][k0+1][r] == inf {
                        dp[i][k0+1][r] = dp[i-1][k0][k1] + a[i];
                    }
                    dp[i][k0+1][r] = max(dp[i][k0+1][r], dp[i-1][k0][k1] + a[i]);
                    // println!("dp[{i}][{}][{r}] = {:?}", k0+1, dp[i][k0+1][r]);
                }
            }
        }
        // debug
        // for i in 0..n {
        //     for k0 in 0..k+1 {
        //         for k1 in 0..k {
        //             println!("dp[{}][{}][{}] = {}", i,k0,k1,dp[i][k0][k1]);
        //         }
        //     }
        // }

        let max_sum = dp[n-1][k][x%k];
        if max_sum == inf {continue}
        let cand = (x - max_sum) / k;
        // println!("(k, max_sum, cand) = {:?} ------", (k, max_sum, cand));
        // for r in 0..k {
        //     println!("dp[n-1][{k}][{r}] = {:?}", dp[n-1][k][r]);
        // }
        // println!("x%k = {:?}", x%k);

        if max_sum == 0 {continue}
        ans = min(ans, cand);
    }
    println!("{}", ans);

    // (x - sum) % k == 0 の必要があり。sumはmaxである必要あり。


}