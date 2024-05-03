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
    // 2024-05-03 10:07-12:21 (2h14min)

    // solve_tle(100, 1);
    // solve_tle(25, 2);
    // solve_tle(314159, 2);

    // // 
    // // solve_tle(325, 3);
    // solve_tle(3258657, 3);
    // solve_tle(3258657, 1);
    // solve_tle(3258657, 2);
    // solve_tle(3250000, 3);
    // solve_tle(3250000, 2);
    // solve_tle(3250000, 1);
    // solve_tle(3250001, 3);
    // solve_tle(3250001, 2);
    // solve_tle(3250001, 1);
    // solve_tle(99, 1);
    // solve_tle(99, 2);
    // solve_tle(99, 3);
    // solve_tle(599, 3);
    // solve_tle(995, 3);
    // solve_tle(55, 1);
    // solve_tle(2, 3);
    // solve_tle(1, 3);
    // solve_tle(1, 2);
    // solve_tle(1, 1);
    // solve_tle(2, 2);
    // solve_tle(10000, 2); // d_hand_04
    // solve_tle(1000, 2); // d_hand_04
    // solve_tle(100, 2); // コーナーケース
    // solve_tle(10, 2);

    // // solve_tle(9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999, 3);
    // println!("-------");
    // // return;

    input! {
        mut nc: Chars,
        k: usize
    }

    // DPっぽい
    // nc.reverse();
    let n = nc.len();
    let mut nu = vec![0; n];
    for i in 0..n {
        nu[i] = nc[i] as usize - '0' as usize;
    }
    // せいぜい100桁
    let mut dp = vec![vec![0; k+1]; n];
    let mut max_num_nonzero = vec![0; n];
    max_num_nonzero[0]=1;
    for i in 1..n {
        if nu[i] == 0 {
            max_num_nonzero[i] = max_num_nonzero[i-1];
        }
        else {
            max_num_nonzero[i] = max_num_nonzero[i-1] + 1;
        }
    }
    // println!("max_num_nonzero = {:?}", max_num_nonzero);


    dp[0][0] = 1;
    dp[0][1] = nu[0];
    // println!("dp[0] = {:?}", dp[0]);
    for i in 1..n {
        dp[i][0] = 1;
        for j in 1..k+1 {
            if max_num_nonzero[i-1] == j - 1 {
            // if i+1 == j {
                // dp[i-1][j-1] がmaxを含む
                dp[i][j] = (dp[i-1][j-1]-1) * 9 + nu[i] + dp[i-1][j];
            }
            else {
                // dp[i-1][j-1] がmaxを含まない
                dp[i][j] = dp[i-1][j-1] * 9 + dp[i-1][j];
            }
        }
        // println!("dp[{i}] = {:?}", dp[i]);
    }
    println!("{}", dp[n-1][k]);
    // 012
    // 325
    // k = 3
    // 32*

    // k=2
    // dp[0桁][2個] = 0
    // dp[0桁][1個] = 3
    // dp[0桁][0個] = 1
    // dp[1桁][2個] = 前回v=3: 2個、前回v=1-2: 2*9個
    // dp[1桁][1個] = 前回v=0: 9個, 前回v=1-3: dp[0][1]
    // dp[1桁][0個] = dp[0][0]
    // dp[2桁][2個] = 前回v=32: 5個、前回v!=32: dp[1][1]*9
    // dp[2桁][1個] = 前回


    // 1以上N以下で、0でない数字がK個あるとき。
    // k=1,2,3

    // 例1:
    // N=100
    // k=1
    // 1-9, 10-90,1=19個
    // k=2
    // 9*9=81個
    // k=3
    // 0

    // 例2:
    // N=25
    // k=2
    // 1* =>9
    // 2* =>5
    // 合計14
}

fn solve_tle(n: usize, k: usize) {
    let mut ans = 0;
    for i in 1..n+1 {
        let num_nonzero = count(i);
        if num_nonzero == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn count(mut n: usize) -> usize {
    let mut num_nonzero = 0;
    loop {
        if n % 10 !=0 {
            num_nonzero += 1;
        }
        n /= 10;
        if n==0{break}
    }
    return num_nonzero
}