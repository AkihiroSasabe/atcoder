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
    // 2023-10-18(Wed.) 19:36-21:00 (1h24min = 84min)
    // 2023-10-21(Sat.) 17:15-17:50 (35min)
    // 119 min
    input! {
        n: usize,
        a: [usize; n]
    }

    if n == 1 {
        println!("{}", 0);
        return;
    }

    // dpっぽい
    // 全探索は、休みの日をiに置くか、置かないかで2^N通りある。

    // cum_a[i] := 0 ~ iまでのaの合計
    let mut cum_a = vec![0; n];
    cum_a[0] = a[0];
    for i in 1..n {
        cum_a[i] = a[i] + cum_a[i-1];
    }

    // rewords[残り日数] := 残り日数での報酬
    let mut rewards = vec![0; n];
    rewards[1] = a[0];
    // rewards[2] = a[0] * 2;
    // rewards[3] = a[0] * 2 + a[1];
    // rewards[4] = (a[0] + a[1]) * 2;
    // rewards[5] = (a[0] + a[1]) * 2 + a[2];
    for remain_days in 2..n {
        rewards[remain_days] = cum_a[remain_days / 2 - 1] * 2 + a[remain_days / 2] * (remain_days % 2);
        // println!("rewards[{remain_days}] = {}", rewards[remain_days]);
    }

    // 0: 勤労、 1: 休み
    // dp[i][0] := iが勤労の日のときの最高生産量 (ただしi以降に休みなし)
    // dp[i][1] := iが休日のときの最高生産量 (ただしi以降に休みなし)
    let mut dp = vec![vec![0; 2]; n];
    
    dp[0][1] = rewards[n-1];
    for i in 1..n {
        for j in 0..i {
            // j: iより前に置いた最後の休日

            // iが勤労日のとき
            dp[i][0] = max(dp[i-1][0], dp[i-1][1]);

            // 残り日数
            let remain_days_i = n - 1 - i; // i以降の出勤日数
            let remain_days_j = n - 1 - j; // j以降の出勤日数
            let remain_days_ij = i - j - 1; // j ~ iの出勤日数
            
            let pre_rewards = dp[j][1] - rewards[remain_days_j]; // 0 ~ j 間の報酬
            let rewards_ij = rewards[remain_days_ij]; // j ~ i 間の報酬
            let rewards_i = rewards[remain_days_i]; // i ~ n-1 間の報酬

            // iが休日のとき (jはiより前に置いた最後の休日)
            dp[i][1] = max(dp[i][1], pre_rewards + rewards_ij + rewards_i);
        }
    }
    println!("{}", max(dp[n-1][0], dp[n-1][1]));



}