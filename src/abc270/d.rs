#![allow(unused_imports)]
// #![allow(dead_code, unused_macros, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, self};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; k]
    }

    // dp[n] :=山にn個の石がある時、 先手が取れる最大の石の数
    let n_max = 10_001;
    let mut dp = vec![0; n_max];
    for stone_num in 0..n+1 {
        for i in 0..k {
            if stone_num < a[i] {continue}
            dp[stone_num] = max(dp[stone_num], a[i]+ stone_num-a[i] - dp[stone_num-a[i]]);
        }
    }
    println!("{}", dp[n]);

    // 2次元の動的計画法だと駄目だった
    // 1次元のテーブルでin-placeにできる。
    // あとDPは遷移するとき、遷移前の状態の値は確定していないといけない。石の数が外側、数列が内側の順にループさせてないからたぶん駄目
    // a.sort();
    // // dp[k][n] := aの第k-1項まで使える時に、高橋が取れる最大の石の数
    // // let mut dp = vec![vec![0; n_max]; k];
    // let mut dp = vec![vec![0; n+1]; k];
    // // dp[0][a[0]] = a[0];
    // for i in 0..k {
    //     for stone_num in 0..n+1 {
    //         // 上からの遷移
    //         if i > 0 {
    //             dp[i][stone_num] = max(dp[i][stone_num], dp[i-1][stone_num]);
    //         }
    //         // 左からの遷移
    //         if stone_num < a[i] {continue}
    //         dp[i][stone_num] = max(dp[i][stone_num], a[i] + stone_num-a[i] -dp[i][stone_num-a[i]]);
    //     }
    // }
    // for i in 0..1 {
    //     println!("{:?}", dp[i]);
    // }
    // println!("{}", dp[k-1][n]);

    // 貪欲法は駄目
    // let mut takahashi = 0;
    // let mut ans = 0;
    // for i in 0..k {
    //     while n >= a[k - 1 - i] {
    //         n = n - a[k - 1 - i];
    //         if takahashi == 0 {
    //             // println!("takashi: {}", a[k - 1 - i]);
    //             ans += a[k - 1 - i];
    //         }
    //         else {
    //             // println!("aoki: {}", a[k - 1 - i]);
    //         }
    //         takahashi = 1 - takahashi;
    //     }
    // }
    // println!("{}", ans);
}