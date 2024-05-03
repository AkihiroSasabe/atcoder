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
    // 2024-05-02 19:57-21:00 (1h3min)
    // 2024-05-02 22:00-23:38 (1h38min)
    // 2h41min
    input! {
        mut nc: Chars
    }
    nc.reverse();
    let mut nu = vec![0; nc.len()];
    for i in 0..nc.len() {
        nu[i] = nc[i] as usize - '0' as usize;
    }

    let INF = 1 << 60;
    let mut dp = vec![vec![INF; 2]; nu.len()];
    // ピッタリで払う
    dp[0][0] = nu[0];

    // 釣りで払う
    dp[0][1] = 10 - nu[0];

    for i in 1..nu.len() {
        // ピッタリで払う
        dp[i][0] = min(dp[i][0], dp[i-1][0] + nu[i]);
        dp[i][0] = min(dp[i][0], dp[i-1][1] + (nu[i] + 1));

        // 釣りで払う
        dp[i][1] = min(dp[i][1], dp[i-1][0] + 10 - nu[i]);
        dp[i][1] = min(dp[i][1], dp[i-1][1] + 10 - (nu[i] + 1));
    }

    let n = nu.len();
    // for i in 0..n {
    //     println!("dp[{i}][0]= {}, dp[{i}][1]= {}", dp[i][0], dp[i][1]);
    // }

    let ans = min(dp[nu.len()-1][0], dp[nu.len()-1][1] + 1);
    println!("{}", ans);

    // let mut num_turi = 0;
    // let mut num_pay = 0;
    // for i in 0..nu.len() {
    //     // println!("nu[{i}] = {} ---- ==== ----", nu[i]);
    //     let mut diff_turi = 0;
    //     let mut diff_pay = 0;
    //     if nu[i] >= 6 {
    //         // 釣り要る
    //         diff_turi += 10 - nu[i];
    //         if i + 1 != nu.len() {
    //             nu[i+1] += 1;
    //         }
    //         else {
    //             diff_pay += 1;
    //         }
    //     }
    //     else if nu[i] == 5 {
    //         if i + 1 != nu.len() {
    //             if nu[i+1] >= 6 {
    //                 // 釣りで払う
    //                 nu[i+1] += 1;
    //                 diff_turi += 10 - nu[i];
    //             }
    //             else {
    //                 // そのまま払う
    //                 diff_pay += nu[i];
    //             }
    //         }
    //         else {
    //             // そのまま払う
    //             diff_pay += nu[i];
    //         }
            
    //     }
    //     else {
    //         // 釣り不要
    //         diff_pay += nu[i];
    //     }
    //     // println!("diff_turi = {:?}", diff_turi);
    //     // println!("diff_pay = {:?}", diff_pay);
    //     num_turi += diff_turi;
    //     num_pay += diff_pay;
        
    // }
    // // println!("nu = {:?}", nu);
    // let ans = num_turi + num_pay;
    // println!("{}", ans);



}