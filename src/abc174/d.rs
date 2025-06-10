#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
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
    // 2025-06-10 19:44-20:20 (36min)
    input! {
        n: usize,   
        c: Chars,   
    }

    // 色々実験していると、
    // 一番右にいる赤を一番左にいる白と入れ替えていけばいいことに気がつく。
    // 入れ替え動作は、色を変える動作よりもコスパがいい場合がある。
    // 証明は難しいけど。
    // 一番右にいる赤を白に変え、一番左にいる白を赤に変えると、2倍の動作が必要になってしまう。

    let mut rs = BinaryHeap::new();
    let mut ws = BinaryHeap::new();

    for i in 0..n {
        if c[i] == 'R' {
            rs.push(i);
        }
        else {
            ws.push(Reverse(i));
        }
    }

    if ws.len() ==0 || rs.len() ==0 {
        println!("0");
        return;
    }

    let mut ans = 0;
    loop {
        let r_max = *rs.peek().unwrap();
        let Reverse(w_min) = *ws.peek().unwrap();
        if r_max < w_min {break}
        rs.pop();
        ws.pop();
        rs.push(w_min);
        ws.push(Reverse(r_max));
        ans += 1;
    }
    println!("{}", ans);



    // WWRRRR
    // RWRRRW
    // RRRRWW
    // WWWRRR
    // RRRWWW
    // WWWRRRR


    // // 2*10^5
    // let inf = 1<< 63;
    // let mut dp = vec![[inf;2]; n];

    // // iまでで、赤い石の左に白がない。
    // // かつiが白: 0
    // // かつiが赤: 1

    // if c[0] == 'R' {
    //     dp[0][0] = 1;
    //     dp[0][1] = 0;
    // }
    // else {
    //     dp[0][0] = 0;
    //     dp[0][1] = 1;
    // }

    // for i in 1..n {
    //     // i を白にする

    //     if s[i] == 'W' {
    //         dp[i][0] = min(dp[i-1][0], dp[i-1][1]);

    //         dp[i][1] = min(dp[i-1][0], dp[i-1][1]);
    //     }
    // }


    // WWWR
    // RWWWR

    // WRWWRWRR
    // W は右にいないといけない。
    // 貪欲に、一番左にいるWを一番右にいるRとチェンジすれば?
    // WRWWRWRR 0
    // RRWWRWRW 1
    // RRRWRWWW 2
    // RRRRWWWW


    // RWR...WRW
    // シャンブルズする価値あり。
    // RRR...WWW

    // WWR...WRW
    // WWWR...WRW

    // Wが左にいて、かつRが右にいる状況(W,R)
    // RWW...WRR


}