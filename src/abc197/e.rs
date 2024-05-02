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
    // 2024-05-02 10:53-12:18 (1h25m)
    input! {
        n: usize,
    }
    let mut x = vec![];
    let mut c = vec![];
    let mut btree = BTreeMap::new();
    for i in 0..n {
        input!{
            xi: isize,
            ci: usize,
        }
        x.push(xi);
        c.push(ci-1);
        // list[ci-1].push(xi);
        btree.entry(ci-1).or_insert(BTreeSet::new()).insert(xi);
    }
    let mut lists = vec![];
    for (ci, set) in btree {
        let mut list = vec![];
        for v in set {
            list.push(v);
        }
        lists.push(list);
    }
    // println!("lists = {:?}", lists);

    // 各色について、終点が左か右にいるときの、最短経路
    let INF = 1 << 60;
    let mut dp = vec![[INF; 2]; lists.len()];

    // 初期色
    let l0 = lists[0][0];
    let r0 = lists[0][lists[0].len()-1];

    // 左
    dp[0][0] = if 0 <= r0 {
        r0.abs() + (r0 - l0).abs()
    } else {
        l0.abs()
    };
    // 右
    dp[0][1] = if 0 < l0 {
        r0.abs()
    } else {
        l0.abs() + (r0 - l0).abs()
    };

    for i in 1..lists.len() {
        // 前回の左と右の位置
        let pre_l = lists[i-1][0];
        let pre_r = lists[i-1][lists[i-1].len()-1];

        // 今回の左と右の位置
        let l = lists[i][0];
        let r = lists[i][lists[i].len()-1];

        // 前回左 -> 今回右 -> 今回左の距離
        let ll = if pre_l <= r {
            (r - pre_l).abs() + (l - r).abs()
        } else {
            (l - pre_l).abs()
        };

        // 前回左 -> 今回左 -> 今回右の距離
        let lr = if pre_l <= l {
            (r - pre_l).abs()
        } else {
            (l - pre_l).abs() + (l - r).abs()
        };

        // 前回右 -> 今回右 -> 今回左の距離
        let rl = if pre_r <= r {
            (r - pre_r).abs() + (l - r).abs()
            // (r - pre_l).abs() + (l - r).abs()
        } else {
            (l - pre_r).abs()
        };
        
        // 前回右 -> 今回左 -> 今回右の距離
        let rr = if pre_r <= l {
            (r - pre_r).abs()
        } else {
            (l - pre_r).abs() + (l - r).abs()
        };

        // 終点が左
        // 左 -> 左
        dp[i][0] = min(dp[i][0], dp[i-1][0] + ll);
        // 右 -> 左
        dp[i][0] = min(dp[i][0], dp[i-1][1] + rl);

        // 終点が右
        // 左 -> 右
        dp[i][1] = min(dp[i][1], dp[i-1][0] + lr);
        
        // 右 -> 右
        dp[i][1] = min(dp[i][1], dp[i-1][1] + rr);
    }

    let fi = lists.len() - 1;
    let last_l = lists[fi][0].abs();
    let last_r = lists[fi][lists[fi].len()-1].abs();

    let ans = min(dp[dp.len()-1][0] + last_l, dp[dp.len()-1][1] + last_r);
    println!("{}", ans);    

}