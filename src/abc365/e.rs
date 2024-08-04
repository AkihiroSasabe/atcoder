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
    // 2024-08-03 21:46-22:40 (54min)
    // 2024-08-04 13:00-13:27 (27min)
    // total 81min
    input! {
        n: usize,
        a: [usize; n]
    }

    // 方針
    // N=5のケースを考える。求めるものは、以下の和。
    // a[1] ^ a[2]
    // a[1] ^ a[2] ^ a[3]
    // a[1] ^ a[2] ^ a[3] ^ a[4]
    // a[1] ^ a[2] ^ a[3] ^ a[4] ^ a[5]
    //        a[2] ^ a[3]
    //        a[2] ^ a[3] ^ a[4]
    //        a[2] ^ a[3] ^ a[4] ^ a[5]
    //               a[3] ^ a[4]
    //               a[3] ^ a[4] ^ a[5]
    //                      a[4] ^ a[5]
    // これは、以下の和 (s[1] + s[2] + s[3] + s[4]) であると考えることができる。
    // s[4] = a[4] ^ a[5]
    // s[3] = a[3] ^ (a[4] + a[4] ^ a[5])
    // s[2] = a[2] ^ (a[3] + a[3] ^ a[4] + a[3] ^ a[4] ^ a[5])
    // s[1] = a[1] ^ (a[2] + a[2] ^ a[3] + a[2] ^ a[3] ^ a[4] + a[2] ^ a[3] ^ a[4] ^ a[5])

    // 一般化すると、
    // s[i-1] = a[i-1] ^ (a[i] + s[i])
    // あとは、これをi=N->1まで、逐次求めていくだけ。

    // debug
    // for i in 0..n {
    //     println!("a[{i}] = {:03b} = {}", a[i], a[i]);
    // }

    // let num_bits = 3; // debug
    let num_bits = 30; // bitの桁数。a[i] <= 10^8 で、 2^30 = 1_073_741_824 > 10^9 だからちょうど良い。

    // dp[i][b] := iビット目がbである個数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; num_bits];
    let mut ans = 0;
    for j in (1..n).rev() {
        for i in 0..num_bits {
            if ((1 << i) & a[j]) != 0 {
                dp[i][1] += 1;
            }
            else {
                dp[i][0] += 1;
            }
        }
        let mut pre = dp.clone();

        for i in 0..num_bits {
            if (1 << i) & a[j-1] != 0 {
                // 今のbitが1
                dp[i][0] = pre[i][1];
                dp[i][1] = pre[i][0];
            }
            else {
                dp[i][0] = pre[i][0];
                dp[i][1] = pre[i][1];
            }
            ans += (1 << i) * dp[i][1];
        }
        // println!("dp = {:?}", dp);
    }

    println!("{}", ans);


}