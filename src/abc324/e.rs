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
    // 2023-10-15 10:05-10:44 (39min, i32でoverflowしていなければ24minでACできた)
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n]
    }
    // 10:07 問題文を理解(2min)
    // 先頭何文字か?
    // 末尾何文字まで持つか?

    // 以下の2行は配列外アクセスをするコードで、RE (実行時エラー)として検出される
    // let mut a = vec![0; n];
    // a[1_000_000_000_000] += 1;

    // AtCoderの実行環境 では、Rustで実行時にOverflowしたときには、REにならずに0で演算を進めてしまう。
    // https://atcoder.jp/contests/ahc011/rules
    // let mut a: u32 = 1;
    // let mut count = 0;
    // while count <= n {
    //     println!("pre_a = {a}");
    //     a *= 1_000_000;
    //     println!("a = {a}");
    //     count += 1;
    // }
    // println!("a = {a}, count = {count}"); // 0がそのまま出力される

    // pre[i]は、S[i]が、Tの先頭何文字を所持するかを示す。
    let mut pre = vec![0; n];
    for i in 0..n {
        let mut si = 0;
        let mut ti = 0;
        loop {
            if si >= s[i].len() || ti >= t.len() {break}
            if s[i][si] == t[ti] {
                ti += 1;
            }
            si += 1;
        }
        pre[i] = ti;
    }

    let mut suf = vec![0; n];
    for i in 0..n {
        let mut si = 0;
        let mut ti = 0;
        let sfin = s[i].len() - 1;
        let tfin = t.len() - 1;
        loop {
            if si >= s[i].len() || ti >= t.len() {break}
            if s[i][sfin - si] == t[tfin - ti] {
                ti += 1;
            }
            si += 1;
        }
        suf[i] = ti;
    }

    // pre_num[i] := Tと先頭からi文字が、一致したSの個数
    // suf_num[i] := Tと末尾からi文字が、一致したSの個数
    let mut pre_num: Vec<usize> = vec![0; t.len() + 1];
    let mut suf_num: Vec<usize> = vec![0; t.len() + 1];
    for i in 0..n {
        pre_num[pre[i]] += 1;
        suf_num[suf[i]] += 1;
    }

    // suf_num_cum[i] := Tと末尾からi文字以上が、一致したSの個数
    let mut suf_num_cum: Vec<usize> = vec![0; t.len() + 1];
    suf_num_cum[t.len()] = suf_num[t.len()];
    for i in (0..t.len()).rev() {
        suf_num_cum[i] = suf_num_cum[i+1] + suf_num[i];
    }

    let mut ans = 0;
    for i in 0..t.len()+1 {
        ans += pre_num[i] * suf_num_cum[t.len()-i];
    }

    // println!("pre = {:?}", pre);
    // println!("suf = {:?}", suf);
    // println!("pre_num = {:?}", pre_num);
    // println!("suf_num = {:?}", suf_num);
    // println!("suf_num_cum = {:?}", suf_num_cum);

    println!("{}", ans);

    
}