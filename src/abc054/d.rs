#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-16 15:57-16:16 (19min)
    input! {
        n: usize,
        mut ma: usize,
        mut mb: usize,
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
            ci: usize, // 値段
        }
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    // 買った薬品は全部使う
    // ma: mb にする
    // 最小予算を求めよ。

    // 1:10
    // 
    // Σai <= 10 * 40 = 400
    // dp[ai][bi]

    // 6_400_000
    let INF = 1<< 60;
    let mut dp = vec![vec![INF; 401]; 401];
    dp[0][0] = 0;
    for i in 0..n {
        let pre_dp = dp.clone();
        for ai in 0..401 {
            for bi in 0..401 {
                if ai + a[i] > 400 || bi + b[i] > 400 {continue}
                dp[ai + a[i]][bi + b[i]] = min(dp[ai + a[i]][bi + b[i]], pre_dp[ai][bi] + c[i]);
            }
        }
    }

    // Dubug
    // for ai in 0..7 {
    //     for bi in 0..7 {
    //         println!("dp[{ai}][{bi}] = {}", dp[ai][bi]);
    //     }
    // }

    let mut ans = INF;
    let mut pro = 1;
    loop {
        let ma2 = ma * pro;
        let mb2 = mb * pro;
        // println!("ma = {}, mb = {:?}", ma, mb);

        if ma2 > 400 || mb2 > 400 {break}
        ans = min(ans, dp[ma2][mb2]);
        pro += 1;
    }

    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }

}