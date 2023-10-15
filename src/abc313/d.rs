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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;

fn main() {
    let input_list: Vec<usize> = read_multi();
    let n = input_list[0];
    let k = input_list[1];
    // インタラクティブなときは、proconioのinput!マクロを使うと動作しないので併用しないこと
    // input! {
    //     n: usize,
    //     k: usize
    // }
    
    // 考察
    // (1). i <= k のとき、以下k+1個のクエリを聞く
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + a[k]
    // a[0] + ____ + a[2] + a[3] + ... + a[k-1] + a[k]
    // a[0] + a[1] + ____ + a[3] + ... + a[k-1] + a[k]
    // a[0] + a[1] + a[2] + ____ + ... + a[k-1] + a[k]
    // ...
    // a[0] + a[1] + a[2] + a[3] + ... + ______ + a[k]
    // a[0] + a[1] + a[2] + a[3] + ... + a[k-1] + ____

    // (1-1). a[0]と、a[j] (j=1,2,...,k) の相等関係を特定
    // (1本目の式とj+1本目の式の相等関係が、a[0]とa[j]の相等関係に等しい)

    // (1-2). S := Σ[i=0, k] a[i] の 偶奇を調べる
    // k + 1個の各クエリについて総和S_sumを計算する
    // S_sum = k * S
    // となるがkは奇数だから、S_sumの偶奇は、Sの偶奇と一致する

    // (1-3). a[0]の偶奇を調べる
    // 1個目のクエリと、Sの関係からa[0]を求める

    // (2)K <= i < n-1 のとき、以下n-1-k個のクエリを聞く
    // a[0] + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + ______ + ______ + ______ + ______ + ...... + ______ + ______
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + a[k+1] + ______ + ______ + ______ + ...... + ______ + ______
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + ______ + a[k+2] + ______ + ______ + ...... + ______ + ______
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + ______ + ______ + a[k+3] + ______ + ...... + ______ + ______
    // ...
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + ______ + ______ + ______ + ______ + ...... + a[n-2] + ______
    // ____ + a[1] + a[2] + a[3] + ... + a[k-1] + ____ + ______ + ______ + ______ + ______ + ...... + ______ + a[n-1] 

    // (2-1). a[0]と、a[j] (j=k+1, k+2, ..., n-1) との関係を特定
    // (2-2). a[i] (i=1,2,...,n-1)を上で求めたa[0]と、a[0]との相等関係から
    // (1)と(2)のトータルのクエリ回数はn-1回ですむ。

    // 求める数列Aを全項nで初期化
    let mut a = vec![n; n];

    // equal_to_a0[i] := a[0]とa[i]の相等関係を記録していく
    let mut equal_to_a0 = vec![false; n];
    equal_to_a0[0] = true;

    // (1) i <= k のとき、k+1個のクエリを聞く
    // s_sum := k+1個の各クエリの排他的論理和を計算していく
    let mut s_sum = 0;
    // 0番目のクエリ
    let t0 = query_1(k, 0);
    s_sum ^= t0;
    for i in 1..k {
        // i番目のクエリ
        let ti = query_1(k, i);

        // 0番目のクエリと、i番目のクエリの相等関係が、a[0]とa[i]の相等関係に等しい
        equal_to_a0[i] = (t0 == ti);
        s_sum ^= ti;
    }
    // k番目のクエリ
    let tk = query_1(k, k);
    equal_to_a0[k] = (t0 == tk);
    s_sum ^= tk;

    // S := Σ[i=0, k] a[i] の 偶奇を調べる
    // S_sum = k * S
    // となるがkは奇数だから、S_sumの偶奇は、Sの偶奇と一致する (∴S_sum=S)

    // a[0]を求める
    // 0番目のクエリと、S(=S_sum)の排他的論理が、a[0]となる
    a[0] = t0 ^ s_sum;

    // (2) K < i < n-1 のとき、以下n-1-k個のクエリを聞く
    for i in k+1..n {
        let ti = query_2(k, i);
        equal_to_a0[i] = (tk == ti);
    }

    // 残りのa[i] (i = 1, 2, ..., n-1)を求める
    print!("! {} ", a[0]);
    for i in 1..n {
        match equal_to_a0[i] {
            true  => a[i] = a[0],
            false => a[i] = a[0] ^ 1
        }
        print!("{} ", a[i]);
    }
    println!("");
    return
}

// インタラクティブな読み込みをする関数 (1行に1変数)
fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

// インタラクティブな読み込みをする関数 (1行に多変数)
fn read_multi() -> Vec<usize> {
    let mut line_string = String::new();
    std::io::stdin().read_line(&mut line_string).expect("入力エラー");
    let line_str_list: Vec<&str> = line_string.split_whitespace().collect();
    let line_usize_list: Vec<usize> = line_str_list.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    return line_usize_list
}

// i=[0, k]について、a[skip]を除いたk個のa[i]についてクエリを投げる関数
fn query_1(k: usize, skip: usize) -> usize {
    print!("? ");
    for i in 0..k+1 {
        if i == skip {continue}
        print!("{} ", i + 1);
    }
    println!("");
    let t = read();
    return t
}

// i = [1, k-1]の全てのa[i]と、x = [k+1, n-1]のいずれかのa[x]についてのクエリを投げる関数
fn query_2(k: usize, target: usize) -> usize {
    print!("? ");
    for i in 1..k {
        print!("{} ", i + 1);
    }
    print!("{} ", target + 1);
    println!("");
    let t = read();
    return t
}