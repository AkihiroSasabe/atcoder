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
    // 2024-05-23 20:51-21:18 (27min, i32で1WA)
    input! {
        l: Chars
    }

    // a + b == a ^ b
    // DPでやるのが良さそう。
    // (0, 0)
    // (1, 0)
    // (0, 1)
    // ならなんでもいい。

    // bitDP
    // 頭から決めていく。


    // a+b = a^b = 10
    // 10,00
    // 00,10

    // 01
    // 00
    let n = l.len();
    // dp[i][1] := 左からi桁目までLと同じやつが、何通りあるか?
    // dp[i][0] := 左からi桁目までL未満が、何通りあるか?
    let mut dp = vec![vec![0; 2]; n];
    dp[0][1] = 2;
    dp[0][0] = 1;
    let modulus: usize = 1_000_000_007;
    for i in 1..n {
        if l[i] == '0' {
            dp[i][1] = dp[i-1][1]; // 1通り
            dp[i][0] = 3 * dp[i-1][0]; // 何を入れてもいい
        }
        else {
            dp[i][1] = 2 * dp[i-1][1];
            dp[i][0] = 3 * dp[i-1][0] + dp[i-1][1];
        }
        dp[i][0] %= modulus;
        dp[i][1] %= modulus;
    }
    let ans = (dp[n-1][0] + dp[n-1][1]) % modulus;
    // println!("dp = {:?}", dp);
    println!("{}", ans);

}