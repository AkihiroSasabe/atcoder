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
use rand::Rng;
fn main() {
    // 2024-10-02 12:41-12:47 (6min)
    // 2024-10-02 20:31-21:21 (50min)
    // Total 56 min
    input! {
        n: usize,
        w: usize,
    }

    let mut ws = vec![];
    let mut vs = vec![];
    for i in 0..n {
        input!{
            wsi: usize,
            vsi: usize,
        }
        ws.push(wsi);
        vs.push(vsi);
    }

    // vs2[i] := 重みが、w[0] + i  の物の価値を格納したリスト
    let mut vs2 = vec![vec![]; 4];
    for i in 0..n {
        vs2[ws[i] - ws[0]].push(vs[i]);
    }

    // vs2[i][j] := 重みが、w[0] + i  の物を、価値が高い順にj個取得したときの、価値の累積和
    for i in 0..4 {
        vs2[i].sort();
        vs2[i].reverse();
        for j in 1..vs2[i].len() {
            vs2[i][j] = vs2[i][j] + vs2[i][j-1];
        }
        vs2[i].insert(0, 0);
    }
    // println!("vs2 = {:?}", vs2);

    // 4種類の重みに、それぞれ何個持たせるかの組合せは、(n+4)_C_(4) となる。
    // N<=100なので、104C4 = 4_598_126 <= 5 * 10^6 で、全探索しても、十分間に合う
    let mut ans = 0;
    for comb in (0..n+4).combinations(4) {
        // println!("comb = {:?} -------", comb);

        // comb[i] := i の仕切りの位置
        // nums[i] := w[0] + i の重みの物の個数
        let mut nums = vec![0; 4];
        nums[0] = comb[0];
        nums[1] = comb[1] - comb[0] - 1;
        nums[2] = comb[2] - comb[1] - 1;
        nums[3] = comb[3] - comb[2] - 1;

        // println!("nums = {:?}", nums);

        let mut cand = 0; // バッグに入れた物の価値の総和
        let mut sum_w = 0; // バッグに入れた物の重みの総和
        let mut is_ok = true; // 実現可能か?
        for i in 0..4 {
            if nums[i] > vs2[i].len()-1 {
                // 与えられたw[0]+iの重さの物の個数が足りなければ、実現不可
                is_ok = false;
                break
            }
            cand += vs2[i][nums[i]];
            sum_w += nums[i] * (ws[0] + i);
        }
        
        // println!("sum_w = {:?}", sum_w);
        if sum_w > w {continue} // 重量オーバー
        if !is_ok {continue} 
        ans = max(ans, cand);
    }
    println!("{}", ans);


    // 解答時の思考経緯
    // wがおもすぎて、ナップザック無理やな

    // n <= 100
    // n　が少ない！ 
    // でも2^100 は無理。

    // w1 <= wi <= w1 + 3, 0-1-2-3 の4種類しか無い。
    // vi <= 10^7

    // dp[v] := 可能な最小の重み　＜ー駄目

    // 貪欲に、価値が高いものから順に取っていけば?
    // 100 * 100 * 100 * 100 でもぎりいけそう

    // 103C3



}