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
    // 2024-10-30 12:21-12:45 (24min)
    // 2024-10-31 20:26-20:32 (6min)
    // 30min
    input! {
        n: usize,
        a: usize,
        b: usize,
        v: [usize; n],
    }
    // 選んだ品物の価値の平均の最大値
    // 選んだ品物の平均が最大となるような品物の選び方が何通りあるか?
    // A個以上、B個以下を選ぶ
    // 1<=A<=B<=N<=50
    // 1<= vi <= 10^15

    // dp[num][0] := num個あるときの価値の最大値
    // dp[num][1] := num個あるときの価値の最大値が、何通りあるか?
    let mut dp = vec![[0; 2]; n+1];
    dp[0][1] = 1;

    for i in 0..n {
        let pre_dp = dp.clone();
        for num in 0..i+1 {
            if dp[num+1][0] < pre_dp[num][0] + v[i] {
                // 更新
                dp[num+1][0] = pre_dp[num][0] + v[i];
                dp[num+1][1] = pre_dp[num][1];
            }
            else if dp[num+1][0] == pre_dp[num][0] + v[i] {
                // 通り数だけ更新
                dp[num+1][1] += pre_dp[num][1];
            }
        }
        // println!("dp = {:?}", dp);
    }
    let mut ans_ave = dp[a][0] as f64 / a as f64;
    let mut ans_val = dp[a][0];
    let mut ans_num = a;
    let mut ans_combinations = dp[a][1];
    for num in a+1..b+1 {
        let ave = dp[num][0] as f64 / num as f64;
        // println!("num = {}, dp[num][0] = {}, ave = {:?}", num, dp[num][0], ave);

        // if ans_ave < ave {
        if ans_val * num < dp[num][0] * ans_num {
            ans_ave = ave;
            ans_val = dp[num][0];
            ans_num = num;
            ans_combinations = dp[num][1];
        }
        // else if ans_ave == ave {
        else if ans_val * num == dp[num][0] * ans_num {
            ans_combinations += dp[num][1];
        }
    }
    println!("{}", ans_ave);
    println!("{}", ans_combinations);

}