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
    // 2024-11-29 20:46-21:00 (14min)
    // 2024-12-03 12:37-12:57 (20min)
    // 2024-12-04 12:18-12:48 (30min, 解説をちらっと見た。bitDPらしい)
    // 2024-12-04 19:43-21:00 (1h17min, 解説みてるのに、わからん)
    // 2024-12-05 20:10-20:33 (23min, https://tutuz.hateblo.jp/entry/2018/07/19/234447 が一番参考になった)
    // Total: 2h44min
    input! {
        n: usize, // ウザギの数
        m: usize, // 観客の数
    }
    // 16! = 20_922_789_888_000 > 2*10^15

    let mut graph = vec![vec![]; n];
    let mut indegs = vec![0; n];
    let mut us = vec![];
    let mut vs = vec![];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
        us.push(ui-1); // 先
        vs.push(vi-1); // 後
    }

    // 全てのウサギ同士の着順関係 を、フロイド・ワーシャル法の要領で求める。
    // 0: map[i][j] := iの後ろにjがいる
    // 1: map[i][j] := iの前にjがいる
    // 2: map[i][j] := iとjに、前後関係はない
    let mut map = vec![vec![2; n]; n];
    for i in 0..m {
        map[us[i]][vs[i]] = 0;
        map[vs[i]][us[i]] = 1;
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if map[i][k] == 0 && map[k][j] == 0 {
                    map[i][j] = 0;
                    map[j][i] = 1;
                } 
            }
        }
    }

    // この問題は、トポロジカルソートの結果が、何通り存在するか答える問題に等しい。
    // dp[mask] := ウサギが、集合mask いるときの着順の組み合わせ数
    // 遷移
    // dp[mask] =  v0が一番右にいる総数 + v1が一番右にいる状況 + ... + v_n-1が一番右にいる状況  
    //          =　Σ[v∈mask] dp[mask - {v}]
    // dp[111] = dp[011] + dp[101] + dp[110]
    // 
    // 例えば、N=3, M=2でエッジが以下のような場合
    // 1->0, 
    // 1->2
    // dp[111] = ({0,1,2}の中で、0が一番右にいる状態数) + ({0,1,2}の中で、1が一番右にいる状態数) + ({0,1,2}の中で2が一番右にいる状態数)
    //         = dp[110] + dp[101] + dp[011] 
    // となるが、エッジより{0,1,2}の中で1が一番右にいる状態 はありえないので、
    // dp[111] = ({0,1,2}の中で0が一番右にいる状態数) + ({0,1,2}の中で2が一番右にいる状態数)
    //         = dp[110] + dp[011] 
    // となる。さらに、
    // dp[110] = ({0,1}の中で0が一番右にいる状態数) + ({0,1}の中で1が一番右にいる状態数)
    //         = dp[100] + dp[010] 
    //         = dp[100] // (1は一番右にいけないので)
    // dp[100] = {0}の中で0が一番右にいる状態数)
    //         = dp[000]
    //         = 1
    // となる。
    // 結局、
    // dp[111] = dp[110] + dp[011] 
    //         = dp[010] + dp[010]
    //         = dp[000] + dp[000]
    //         = 1 + 1
    //         = 2
    // となる。

    let mut dp: Vec<isize> = vec![0; 1 << n];
    dp[0] = 1; // 空集合のとき、着順は1通り。
    for mask in 0..1<<n {
        for i in 0..n {
            // 既にいるなら、スキップ
            if mask & (1 << i) != 0 {continue}

            // i は一番右になりうるか?
            let mut is_i_right = true;
            for v in 0..n {
                // v ∈ mask とする。 i -> v となるvが1個でもあれば、iは一番右にいけない
                if mask & (1 << v) != 0 {
                    if map[v][i] == 1 {
                        is_i_right = false;
                        break
                    }
                }
            }
            if !is_i_right {continue}
            let next_mask = mask | (1 << i);
            dp[next_mask] += dp[mask];
        }
    }
    // for mask in 0..1<<n {
    //     println!("dp[{:03b}] = {:?}", mask, dp[mask]);
    // }
    println!("{}", dp[(1 << n) - 1]);
}    


// fn solve_wa() {
//     // 2^16 = 65_536 <= 10^5
//     // dp[mask] := 何通りあるか?
//     let mut dp: Vec<usize> = vec![0; 1 << n];
//     dp[0] = 1;
//     for mask in 0..1<<n {
//         // let pre_dp = dp.clone();
//         for i in 0..n {
//             if mask & (1 << i) != 0 {continue} // すでにmaskがiを含んでいる
//             let next_mask = mask | (1 << i);
//             // println!("mask = {:03b}", mask);
//             // println!("next_mask = {:03b}", next_mask);
//             if dp[next_mask] != 0 {continue} // 遷移先を既に計算済み

//             let mut diff = 1;
//             for j in 0..n {
//                 if mask & (1 << j) == 0 {continue} // maskにjがいない
//                 // j と i に、前後関係がなければいい。
//                 if map[i][j] == 2 {
//                     diff += 1;
//                 }
//             }
//             dp[next_mask] += dp[mask] * diff;
//         }
//     }
//     // for mask in 0..1<<n {
//     //     println!("dp[{:03b}] = {:?}", mask, dp[mask]);
//     // }
    
//     println!("{}", dp[(1<<n)-1]);
// }