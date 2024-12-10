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
    // 2024-12-09 20:20-21:00 (40min, 大まかな方針は思いついたが、サンプルと答え合わず。)
    // 2024-12-10 12:33-12:58 (25min, デバッグ。原因1: 同色からの遷移の実装が漏れていた。原因2:xiとするべきところを、x[i]としていたため、)
    // Total: 65min
    input! {
        n: usize, // 商品の数
        x: usize, // 合計価格
        k: usize, // 色の満足度の定数
    }
    let mut cpu = vec![];
    for i in 0..n {
        input!{
            pi: usize, // 価格
            ui: usize, // 効用
            ci: usize, // 色
        }
        cpu.push((ci, pi, ui));
    }
    cpu.sort();

    // 満足度 = S + T * K (S := sum(ui), T := 色の種類数)
    // N <= 500
    // X <= 50_000 = 5 * 10^4
    // K <= 10^9
    // NX で解きなさい NX <= 25 * 10^6 = 2.5 * 10^7

    // println!("cpu = {:?}", cpu);

    // let mut dp = vec![vec![0; 5  * x+1]; n+1];
    let mut dp = vec![vec![0; x+1]; n+1];
    let mut pre_color_ind = 0;
    // 同じ色ごとにまとめて、解けばいいのでは?
    for i in 0..n {
        let ci = cpu[i].0;
        let pi = cpu[i].1;
        let ui = cpu[i].2;
        
        // 前回の色と異なるか?
        let is_diff =  i == 0 || ci != cpu[i-1].0;
        dp[i+1] = dp[i].clone();
        // println!("i = {i}, pre_color_ind = {:?} --------------", pre_color_ind);
        for x0 in 0..x+1 {
            if x0 + pi > x {continue}
            if is_diff {
                dp[i+1][x0 + pi] = max(dp[i+1][x0 + pi], dp[i][x0] + ui + k);
            }
            else {
                // 同色からの遷移
                dp[i+1][x0 + pi] = max(dp[i+1][x0 + pi], dp[i][x0] + ui);
                // 異色からの遷移
                dp[i+1][x0 + pi] = max(dp[i+1][x0 + pi], dp[pre_color_ind][x0] + ui + k);
            }
        }    
        if i != n - 1 && ci != cpu[i+1].0 {
            pre_color_ind = i + 1;
        }
        // println!("dp[{}] = {:?}", i+1, dp[i+1]);
    }
    println!("{}", dp[n][x]);
}


