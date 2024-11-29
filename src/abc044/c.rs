#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
    // 2024-11-28 20:28-20:49 (21min)
    input! {
        n: usize,
        a: usize,
        x: [usize; n],
    }

    let mut sum_x = 0;
    for i in 0..n {
        sum_x += x[i];
    }

    // 平均をAにしたい。
    
    // dp[i][手札の数T][手札の和S] := i番目までのカードで、(T,S)を実現出来る和
    // 50^4 = 6_250_000 。間に合いそう。
    let mut dp = vec![vec![vec![0; sum_x *3]; n + 10]; n + 10];
    dp[0][0][0] = 1;
    for i in 0..n {
        dp[i+1] = dp[i].clone();
        for num in 0..n+1 {
            for sum in 0..sum_x+1 {
                dp[i+1][num+1][sum+x[i]] += dp[i][num][sum]; 
            }
        }
    }

    // println!("dp[n] = {:?}", dp[n]);

    // for i in 0..n+1 {
    //     for num in 0..n+1 {
    //         println!("dp[{i}][{num}] = {:?}", dp[i][num]);
    //     }
    // }


    let mut ans: usize = 0;
    for num in 1..n+1 {
        if num * a >= sum_x *3 {continue}
        ans += dp[n][num][num * a];
    }
    println!("{}", ans);

}