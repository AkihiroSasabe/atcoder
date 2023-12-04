#![allow(unused_imports)]
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
    // 2023-12-02 18:26-19:06 (30min)
    // 2023-12-03 10:36-10:46 (10min)
    // 40min
    input! {
        n: usize,
        c: usize,
        ta: [[usize; 2]; n]
    }
    // 1_073_741_824
    const MAX_KETA: usize = 30;

    // cum[操作i][桁k][初期状態s] := k桁目の初期状態がsのときの、i回目の操作後のk桁目の状態
    let mut cum = vec![[[0, 0]; MAX_KETA]; n];

    for i in 0..n {
        let t = ta[i][0];
        let a = ta[i][1];
        for keta in 0..MAX_KETA {
            let a_keta = (a & (1 << keta)) >> keta;

            let pre_0;
            let pre_1;
            if i == 0 {
                pre_0 = 0;
                pre_1 = 1;
            }
            else {
                pre_0 = cum[i-1][keta][0];
                pre_1 = cum[i-1][keta][1];
            }
            
            if t == 1 {
                cum[i][keta][0] = pre_0 & a_keta;
                cum[i][keta][1] = pre_1 & a_keta;
            }
            else if t == 2 {
                cum[i][keta][0] = pre_0 | a_keta;
                cum[i][keta][1] = pre_1 | a_keta;
            }
            else {
                cum[i][keta][0] = pre_0 ^ a_keta;
                cum[i][keta][1] = pre_1 ^ a_keta;
            }
        }

    }
    // println!("cum = {:?}", cum);

    // now := 2進数における c のi桁目を表す。
    let mut now = vec![0; MAX_KETA];
    for j in 0..MAX_KETA {
        if (1 << j) & c != 0 {
            now[j] = 1;
        }
    }
    // println!("now = {:?}", now);

    for i in 0..n {
        // ans := now を 2進数から、10進数に戻したもの。
        let mut ans = 0;

        // 1桁ずつ now にi番目の演算を施す。
        for j in 0..MAX_KETA {
            // println!("now[j] = {:?}", now[j]);
            now[j] = cum[i][j][now[j]];
            if now[j] == 1 {
                ans += 1 << j;
            }
        }
        println!("{}", ans);
    }

}