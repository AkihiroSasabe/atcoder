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
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    let mut cost = vec![];
    let mut z_sum = 0;
    let mut taka_sum = 0;
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
            z_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
        z.push(z_i);
        z_sum += z_i;
        if x_i < y_i {
            let cost_i = (y_i + x_i) / 2 + 1 - x_i;
            cost.push(vec![cost_i, i as isize]);
        }
        else {
            taka_sum += z_i;
        }
    }
    cost.sort();
    // println!("cost = {:?}", cost);
    let victory_num = z_sum / 2 + 1;
    if taka_sum >= victory_num {
        println!("0");
        return;
    }

    // println!("victory_num - taka_sum = {}", victory_num - taka_sum);

    let INF = 1 << 60;
    let MAX_GISEKI = 300_000;
    let MAX_SENKYOKU_NUM = 300;
    // let MAX_GISEKI = 100;
    // let MAX_SENKYOKU_NUM = n;

    // dp[選挙区i][得る議席] = 動かす必要がある最低人数
    let mut dp = vec![vec![INF; MAX_GISEKI]; MAX_SENKYOKU_NUM];
    for i in 0..cost.len() {
        dp[i][0] = 0;
    }
    let cost_i = cost[0][0];
    let area_i = cost[0][1] as usize;
    let plus_i = z[area_i];
    dp[0][plus_i as usize] = cost_i;
    // println!("dp[0]={:?}", dp[0]);

    for i in 1..cost.len() {
        let cost_i = cost[i][0];
        let area_i = cost[i][1] as usize;
        let plus_i = z[area_i];
        for giseki in 0..100_001 {
            dp[i][giseki] = min(dp[i][giseki], dp[i-1][giseki]);
            dp[i][giseki + plus_i as usize] = min(dp[i][giseki + plus_i as usize], dp[i-1][giseki] + cost_i);
        }
        // println!("dp[{}]={:?}",i, dp[i]);
        // println!("dp[{}][65]={:?}",i, dp[i][65]);
    }

    let mut ans = INF;
    for giseki in ((victory_num-taka_sum) as usize)..MAX_GISEKI {
        ans = min(ans, dp[cost.len()-1][giseki]);
        // println!("giseki={} {}", giseki, dp[cost.len()-1][giseki]);
    }
    println!("{}", ans);

}