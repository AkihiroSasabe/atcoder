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
    // 2024-03-16 11:15-12:48 (1h33min)
    input! {
        la: usize,
        lb: usize,
        mut k: usize,
    }
    // 60C30
    let a = 0;
    let b = 1;
    // dp[文字数][Aの個数][先頭がA or B] := それを満たす個数
    let mut dp = vec![vec![[0; 2]; la + 1]; la + lb + 1];

    // 例 (a,b,k)=(2,2,4)
    // 順位
    // 1 a a b b
    // 2 a b a b
    // 3 a b b a
    // 4 b a a b
    // 5 b a b a
    // 6 b b a a

    if la >= 1 {
        dp[1][1][a] = 1;
    }
    if lb >= 1 {
        dp[1][0][b] = 1;
    }
    // println!("dp[1] = {:?}", dp[1]);

    for i in 2..(la + lb + 1) {
        for ai in 0..min(i, la+1) {
            // ai: 遷移元の ai の個数
            // println!("i = {}, ai = {:?}", i, ai);

            if ai + 1 <= la {
                // println!("---- i = {}, ai = {:?}", i, ai);
                dp[i][ai+1][a] += dp[i-1][ai][a];
                dp[i][ai+1][a] += dp[i-1][ai][b];
            }
            
            if i - ai <= lb {
                dp[i][ai][b] += dp[i-1][ai][a];
                dp[i][ai][b] += dp[i-1][ai][b];
            }
        }
        // println!("dp[{i}] = {:?}", dp[i]);
    }

    let mut num_a = la;
    for i in (1..la+lb+1).rev() {
        // 先頭がaの個数
        let mut sum = 0;
        sum += dp[i][num_a][0];

        // println!("");
        // println!("-------");
        // println!("sum = {:?}", sum);
        // println!("k = {:?}", k);
        // println!("num_a = {:?}", num_a);

        if sum >= k {
            print!("a");
            num_a -= 1;
        }
        else {
            k -= sum;
            print!("b");
        }
    }

    // 118_264_581_564_861_424


}