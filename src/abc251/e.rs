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
    // 2023-12-06 16:49-17:21 (32min)
    input! {
        n: usize,
        a: [usize; n]
    }

    const INF: usize = 1 << 60;

    // 1本目を選ばない
    let mut dp0 = vec![vec![INF; 2]; n];
    dp0[0][0] = 0;

    // 1本目を選んでいる
    let mut dp1 = vec![vec![INF; 2]; n];
    dp1[0][1] = a[0];

    for i in 1..n-1 {
        // i本目の辺を選ばない
        dp0[i][0] = dp0[i-1][1];
        dp1[i][0] = dp1[i-1][1];

        // i本目の辺を選ぶ
        dp0[i][1] = min(dp0[i-1][0] + a[i], dp0[i-1][1] + a[i]);
        dp1[i][1] = min(dp1[i-1][0] + a[i], dp1[i-1][1] + a[i]);
    }
    // println!("dp0 = {:?}", dp0);
    // println!("dp1 = {:?}", dp1);

    // 最後の辺を選ばない
    let mut ans = dp1[n-2][1];

    // 最後の辺を選ぶ
    ans = min(ans, dp0[n-2][0] + a[n-1]);
    ans = min(ans, dp0[n-2][1] + a[n-1]);
    ans = min(ans, dp1[n-2][0] + a[n-1]);
    ans = min(ans, dp1[n-2][1] + a[n-1]);

    println!("{}", ans);
    
}