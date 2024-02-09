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
    // 2024-02-09 12:16-12:49 (33min)
    // 2024-02-09 18:49-19:38 (49min)
    // total: 82min
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let modulus: u32 = 998_244_353;
    // 全探索は2^N = 2^5000 だからTLE
    // solve_brute_force(n, a, b, modulus);

    let a_max = *a.iter().max().unwrap();
    let b_max = *b.iter().max().unwrap();
    let ab_max = max(a_max, b_max);

    // aの順番にソートする。
    let mut ab = vec![];
    for i in 0..n {
        ab.push([a[i], b[i]]);
    }
    ab.sort();
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        a.push(ab[i][0]);
        b.push(ab[i][1]);
    }


    // dp[i][最大][和] というdpはどうかな? -> 間に合わない N * ab_max * ab_max でTLE する
    // dp[i][和][可否] というdpはどうかな? 1:可、0:不可 -> TLEしない。
    // dp[i][w][0] := i番目までに出てくる数列を使って、和がwとなり、max(a) < Σb となるものの個数
    // dp[i][w][1] := i番目までに出てくる数列を使って、和がwとなり、max(a) >= Σb となるものの個数
    let mut dp = vec![vec![0; 2]; ab_max + 1];
    // dpの初期化 

    // i = 0 のときの遷移
    dp[0][0] = 1;
    dp[0][1] = 0;
    if b[0] <= a[0] {
        dp[b[0]][0] = 0; 
        dp[b[0]][1] = 1;
    }
    else {
        dp[b[0]][0] = 1; 
        dp[b[0]][1] = 0;
    }
    // i > 0 のときの遷移
    for i in 1..n {
        // let pre_dp = dp;
        let mut pre_dp = vec![vec![0;2]; ab_max + 1];
        swap(&mut dp, &mut pre_dp);
        for w in 0..ab_max+1 {
            // 可なやつはそのまま可でok
            dp[w][1] += pre_dp[w][1];
            dp[w][1] %= modulus;

            // 不可なやつは、そのまま不可でok
            dp[w][0] += pre_dp[w][0];
            dp[w][0] %= modulus;

            if w + b[i] > ab_max {continue}
            if w + b[i] <= a[i] {
                dp[w + b[i]][1] += pre_dp[w][0];
                dp[w + b[i]][1] %= modulus;
                dp[w + b[i]][1] += pre_dp[w][1];
                dp[w + b[i]][1] %= modulus;
            }
            else {
                dp[w + b[i]][0] += pre_dp[w][0];
                dp[w + b[i]][0] %= modulus;
                dp[w + b[i]][0] += pre_dp[w][1];
                dp[w + b[i]][0] %= modulus;
            }
        }
    }
    let mut ans = 0;
    for w in 1..ab_max+1 {
        ans += dp[w][1];
        ans %= modulus;
    }
    // println!("{}", (ans + modulus - 1) % modulus);
    println!("{}", ans);


    // println!("dp[n-1] = {:?}", dp[n-1]);

    // // Biの和は有限
    // let mut dp = vec![vec![0; ab_max + 1]; n];

// テストケース
// 5
// 1 3 3 4 5
// 1 2 2 3 5

//  sigma(b): 0  1  2  3
// dp[n-1] = [1, 1, 2, 3, 2, 4]
// a_ind = [[1, 0], [3, 1], [3, 2], [4, 3], [5, 4]]

// 答え: 8

// dp[i][最大][和] というdpはどうかな?



}


fn solve_brute_force(n: usize,  a: Vec<usize>, b: Vec<usize>, modulus: usize) {
    // 全探索作る
    let mut ans: usize = 0;
    for bit in 1..1<<n {
        let mut a_max = 0;
        let mut b_sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                a_max = max(a_max, a[i]);
                b_sum += b[i];
            }
        }
        if a_max >= b_sum {
            ans += 1;
            ans %= modulus;
        }
    }
    println!("{}", ans);


}