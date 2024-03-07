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
    // 2024-03-06 20:32-22:00 (1h28min)
    // 2024-03-07 18:38-19:35 (57min)
    // Total 2h25min
    input! {
        n: usize,
        a: [usize; n]
    }
    // BとCは異なる
    // rb = (A[B[1]] + A[B[2]] + ... + A[B[x-1]]) % 200
    // rc = (A[C[1]] + A[C[2]] + ... + A[C[y-1]]) % 200

    // 問題理解に 6 min

    // 復元dp。
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 200]; n];
    dp[0][0] += 1;
    dp[0][a[0] % 200] += 1;
    // println!("dp[0][0] = {:?}", dp[0][0]);
    // println!("dp[0][{}] = {:?}", a[0] % 200, dp[0][a[0] % 200]);
    for i in 1..n {
        // println!("==== {} ====", i);
        for r in 0..200 {
            // a[i]を足さない場合
            dp[i][r] += dp[i-1][r];

            // a[i]を足す場合
            dp[i][(r + a[i]) % 200] += dp[i-1][r];
        }
        // println!("dp[{i}] = {:?}", dp[i]);
        // for r in 0..200 {
        //     if dp[i][r] > 0 {
        //         println!("dp[{i}][{r}] = {}", dp[i][r]);
        //     }
        // }
    }

    // DP復元
    let mut b = vec![];
    let mut c = vec![];
    let mode = 0;
    for r in (0..200).rev() {
        if dp[n-1][r] >= 2 {
            dfs(&dp, n-1, r, &a, &mut b, &mut c, mode);
            break
        }
    }

    b.sort();
    c.sort();
    // println!("b = {:?}", b);
    // println!("c = {:?}", c);

    if b.len() != 0 && c.len() != 0 {
        println!("Yes");
        print!("{} ", b.len());
        for i in 0..b.len() {
            print!("{} ", b[i] + 1);
        }
        println!("");
        print!("{} ", c.len());
        for i in 0..c.len() {
            print!("{} ", c[i] + 1);
        }
    }
    else {
        println!("No");
    }
}

/// DP復元用の再帰関数
fn dfs(dp: &Vec<Vec<usize>>, ind: usize, r: usize, a: &Vec<usize>, b: &mut Vec<usize>, c: &mut Vec<usize>, mut mode: usize) {
    // println!("-------- ind = {ind}, mode = {mode}, r ={r} --------");
    // dp[ind][r] が存在するとき、
    // ind を入れるか、入れないか、判断する

    if ind == 0 {
        if r == a[0] % 200 {
            if mode == 1 {
                b.push(ind);
            }
            else if mode == 2 {
                c.push(ind);
            }
            else {
                b.push(ind);
            }
        }
        return
    }

    let r2: usize = (200 + r - a[ind] % 200) % 200;
    // println!("dp[{}][r={}]={}, dp[{}][r2={}]={}", ind-1, r, dp[ind-1][r], ind-1, r2, dp[ind-1][r2]);
    // println!("");
    // b だけ入れる
    if mode == 1 {
        if dp[ind-1][r2] >= 1 {
            b.push(ind);
            dfs(&dp, ind-1, r2, a, b, c, mode);
        }
        else {
            dfs(&dp, ind-1, r, a, b, c, mode);
        }
        return
    }
    // c だけ入れる
    else if mode == 2 {
        if dp[ind-1][r2] >= 1 {
            c.push(ind);
            dfs(&dp, ind-1, r2, a, b, c, mode);
        }
        else {
            dfs(&dp, ind-1, r, a, b, c, mode);
        }
        return
    }

    // b と c を両方入れないといけないモード
    if dp[ind-1][r] >= 1 && dp[ind-1][r2] >= 1 {
        b.push(ind);

        // b だけ入れるモードでDFS開始
        mode = 1;
        dfs(&dp, ind-1, r2, a, b, c, mode);

        // c だけ入れるモードでDFS開始
        mode = 2;
        dfs(&dp, ind-1, r, a, b, c, mode);
    }
    else if dp[ind-1][r2] >= 2 {
        b.push(ind);
        c.push(ind);
        dfs(&dp, ind-1, r2, a, b, c, mode);
    }
    else {
        dfs(&dp, ind-1, r, a, b, c, mode);
    }
}