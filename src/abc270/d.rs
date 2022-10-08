#![allow(unused_imports)]
// #![allow(dead_code, unused_macros, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        mut n: usize,
        k: usize,
        a: [usize; k]
    }

    let mut dp = vec![0; n+1];
    for i in 1..(n+1) {
        for j in 0..k {
            // let j = k - 1 - jj;
            if i < a[j] {continue}
            dp[i] = max(dp[i], a[j]);

            if i - a[j] < dp[i-a[j]] {continue}
            dp[i] = max(dp[i], a[j] + dp[i - a[j] - dp[i-a[j]]]);            
        }
    }
    println!("{:?}", dp);
    println!("{}", dp[n]);

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