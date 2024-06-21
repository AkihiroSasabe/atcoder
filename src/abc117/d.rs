#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
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
    // 2024-06-20 19:35-20:21 ( 46 min)
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    // k = 1001
    //     1111

    let num_bits = 50;
    // sum[j][0] := x の jビット目が0のときに、得られる和
    // sum[j][1] := x の jビット目が1のときに、得られる和
    let mut sum: Vec<[usize; 2]> = vec![[0; 2]; num_bits];
    let mut max_bit_of_ok = 0;
    for i in 0..n {
        for j in 0..num_bits {
            if a[i] & (1 << j) != 0 {
                sum[j][0] += 1 << j;
            }
            else {
                if (1 << j) <= k {
                    sum[j][1] += 1 << j;
                } 
            }
            
            if (1 << j) <= k {
                max_bit_of_ok = max(max_bit_of_ok, j);
            } 
        }
    }

    // dp[i][0] := 左端のビットからi番目のビットが、kより小さいやつで、和が最大のもの。
    // dp[i][1] := 左端のビットからi番目のビットが、kと等しいやつで、和が最大のもの。
    let mut dp = vec![vec![0; 2]; num_bits+1];
    for i in (0..num_bits).rev() {
        // 1 << i が k と等しい場合
        if i < max_bit_of_ok {
            dp[i][0] = dp[i+1][0] + max(sum[i][0], sum[i][1]);
        }
        if (1 << i) & k != 0 {
            dp[i][0] = max(dp[i][0], dp[i+1][1] + sum[i][0]);
        }

        if (1 << i) & k != 0 {
            dp[i][1] = dp[i+1][1] + sum[i][1];
        }
        else {
            dp[i][1] = dp[i+1][1] + sum[i][0];
        }
    }
    // println!("dp = {:?}", dp);
    let ans = max(dp[0][0], dp[0][1]);
    println!("{}", ans);


}