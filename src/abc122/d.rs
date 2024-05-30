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
    // 2024-05-28 20:01-22:04 (2h3m)
    input! {
        n: usize
    }
    solve_kaisetu(n);
    // solve_mine(n);

}

fn solve_kaisetu(n: usize) {
    let modulus = 1_000_000_007;
    let mut dp = vec![[[[[0; 4]; 4]; 4]; 4]; n];

    if n == 3 {
        println!("61");
        return 
    }

    // _AGC
    // _GAC
    // _ACG
    
    // A_GC
    // AG_C

    // AGC_
    // GAC_
    // ACG_
    // が駄目

    // 初期化
    for c0 in 0..4 {
        for c1 in 0..4 {
            for c2 in 0..4 {
                for c3 in 0..4 {
                    if c1 == 0 && c2 == 2 && c3 == 1 {continue} // _AGC
                    if c1 == 2 && c2 == 0 && c3 == 1 {continue} // _GAC
                    if c1 == 0 && c2 == 1 && c3 == 2 {continue} // _ACG
                    if c0 == 0 && c2 == 2 && c3 == 1 {continue} // A_GC
                    if c0 == 0 && c1 == 2 && c3 == 1 {continue} // AG_C

                    if c0 == 0 && c1 == 2 && c2 == 1 {continue} // AGC_
                    if c0 == 2 && c1 == 0 && c2 == 1 {continue} // GAC_
                    if c0 == 0 && c1 == 1 && c2 == 2 {continue} // ACG_
                    dp[3][c0][c1][c2][c3] = 1;
                }
            }
        }
    }

    // 遷移
    for i in 4..n {
        for c0 in 0..4 {
            for c1 in 0..4 {
                for c2 in 0..4 {
                    for c3 in 0..4 {
                        for c4 in 0..4 {
                            if c2 == 0 && c3 == 2 && c4 == 1 {continue} // _AGC
                            if c2 == 2 && c3 == 0 && c4 == 1 {continue} // _GAC
                            if c2 == 0 && c3 == 1 && c4 == 2 {continue} // _ACG
                            if c1 == 0 && c3 == 2 && c4 == 1 {continue} // A_GC
                            if c1 == 0 && c2 == 2 && c4 == 1 {continue} // AG_C

                            if c1 == 0 && c2 == 2 && c3 == 1 {continue} // AGC_
                            if c1 == 2 && c2 == 0 && c3 == 1 {continue} // GAC_
                            if c1 == 0 && c2 == 1 && c3 == 2 {continue} // ACG_
                            dp[i][c1][c2][c3][c4] += dp[i-1][c0][c1][c2][c3];
                            dp[i][c1][c2][c3][c4] %= modulus;
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for c0 in 0..4 {
        for c1 in 0..4 {
            for c2 in 0..4 {
                for c3 in 0..4 {
                    ans += dp[n-1][c0][c1][c2][c3];
                    ans %= modulus;
                }
            }
        }
    }

    println!("{}", ans);

}

fn solve_mine(n: usize) {
    // 長さNの文字列の数の mod 10^9+7
    // (1) A, C, G, T 以外の文字を含まない。
    // (2) AGC を部分文字列として含まない
    // (3) 隣接する 2 文字の入れ替えを 1 回行うことで上記の条件に違反させることはできない。
    // (3)は、 (ACG, GAC, CGA) も駄目と行っているのと同じ。
    // 整理すると、
    // (1) A, C, G, T 以外の文字を含まない。
    // (2) (AGC, ACG, GAC, CGA) は部分文字列として駄目
    // 1歩前と今の2文字の状態 -> 4*4 = 16 パターン
    // ACG 
    // AGC 
    // CGA <- これは許される
    // GAC 


    // DPで行けるやろ
    // AC -> 
    // AG -> {AGA, AGC, AGG, AGT}
    // GA -> 

    // AAとAを区別できない...ということはないか?

    let modulus = 1_000_000_007;
    
    // dp[]
    let mut dp = vec![vec![0; 16]; n];
    for i in 0..16 {
        dp[1][i] = 1;
    }
    for i in 2..n {
        println!("---- i = {} ----", i);
        for mask in 0..16 {
            let mask_str = mask_to_str(mask);
            println!("mask_str = {:?}", mask_str);
            for j in 0..4 {
                // 前2個がACのとき
                if mask == (0 + 1 * 4) {
                    // Gが駄目
                    if j == 2 {continue}
                }
                // 前2個がAGのとき
                else if mask == (0 + 2 * 4) {
                    // Cが駄目
                    if j == 1 {continue}
                }
                // 前2個がGAのとき
                else if mask == (2 + 0 * 4) {
                    // Cが駄目
                    if j == 1 {continue}
                }
                let n_mask = (mask / 4) + j * 4;
                let n_mask_str = mask_to_str(n_mask);
                dp[i][n_mask] += dp[i-1][mask];
                dp[i][n_mask] %= modulus;
            }
        }
    }
    // println!("dp = {:?}", dp);
    for mask in 0..16 {
        // println!("dp[2][{}] = {:?}", mask_to_str(mask), dp[2][mask]);
        println!("dp[3][{}] = {:?}", mask_to_str(mask), dp[3][mask]);
    }
    let mut ans: usize = 0;
    for i in 0..16 {
        ans += dp[n-1][i];
        ans %= modulus;
    }
    println!("{}", ans);


}

fn mask_to_str(mask: usize) -> String {
    let string = ['A', 'C', 'G', 'T'];
    let i0 = mask % 4;
    let i1 = mask / 4;
    let out = format!("{}{}", string[i0], string[i1]);
    return out
}