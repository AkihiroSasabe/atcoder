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
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let N_INF = (-1) * (1 << 60);
    // let INF = 1 << 60;
    let mut dp = vec![vec![N_INF; m]; n];
    
    dp[0][0] = a[0];
    for i in 1..n {
        dp[i][0] = max(dp[i-1][0], a[i]);
    }

    // dp[i:使用可能なインデックス][部分列のj番目] = sigma(j*Bj)の最大値
    for j in 1..m {
        let mut max_index = 0;
        for i in 1..n {
            if i < j {continue}
            dp[i][j] = max(dp[i-1][j], dp[i-1][j-1] + (j as isize + 1) * a[i]);
        }
    }

    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    println!("{}", dp[n-1][m-1]);
}