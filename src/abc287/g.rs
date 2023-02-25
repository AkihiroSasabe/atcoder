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
    // input! {
        
    // }
    // 2023/02/07 12:11-
    // 蟻本p329 DNA Repair (POJ 3691) 
    // 文字列を華麗に扱う: 複数文字列の場合
    // 問題: 元となる文字列S
    // Sのある場所の文字を別の文字に変更する。
    // n個の禁止文字列: P1, p2, ..., Pn を含まないようにしたい。
    // 最低何回の動作でできるか。無理なら-1を返す。
    // ただし、全ての文字列は'A','G','C','T'の4種類から成る。

    let AVAILABLE_CHARS= vec!['A','G','C','T'];

    // 文字列S
    let s = String::from("AAAG");
    let p = vec![String::from("AAA"), String::from("AAG")];
    // 答え1

    // let s = String::from("TGAATG");
    // let p = vec![String::from("A"), String::from("TG")];
    // // 答え4

    // let s = String::from("AGT");
    // let p = vec![String::from("A"), String::from("G"), String::from("C"), String::from("T")];
    // // 答え-1

    let n = p.len();

    // 1文字加えた際に移動する先の状態
    // 状態0: 下記以外
    // 状態1: **A
    // 状態2: *AT
    // 状態3: ATC
    // 状態4: ATCG (4からの移行先は用意されていない)
    // next[i][j] =: 状態i(=s[0..i])に、AVAILABLE_CHARS[j]を付け加えた際に移動する状態
    // 例えば、状態1(i=1)に、T(j=3)を加えた状態は、next[1][3] = 2 (=*AT) になる.
    // let max_state = n * 19 + 1;
    let max_state = 10; // デバッグ用
    let mut next = vec![vec![0; AVAILABLE_CHARS.len()]; max_state];
    
    // 到達してはいけない状態であるか
    let mut ng = vec![false; max_state];

    // dp[対象の文字数i][末尾の状態番号j] := 文字数がiのとき、末尾の状態番号jは何個あるか?
    // 例えばdp[2][1]なら、2文字で"*A"なので、AA, TA, CA, GAの4種類
    let mut dp = vec![vec![0; max_state]; s.len()+1];

    // 前処理
    // まずは先頭となる文字列を全て列挙
    let mut prefix = vec![];
    let mut hash = HashMap::new();
    for i in 0..n {
        for j in 0..p[i].len() {
            // 重複する文字列は除外
            if !hash.contains_key(&p[i][0..j]) {
                hash.insert(&p[i][0..j], 0);
                prefix.push(&p[i][0..j]);
            }
        }
    }

    let k = prefix.len();

    // 各状態の情報を計算
    for i in 0..k {
        // 末尾が禁止パターンに一致していたらこの状態は到達していけない
        for j in 0..n {
            ng[i] |= p[j].len() <= prefix[i].len()
                    && &prefix[i][prefix[i].len()-p[j].len()..prefix[i].len()] == p[j];
        }
        for j in 0..AVAILABLE_CHARS.len() {
            // 1文字加えた文字列
            let mut pushed = String::from(prefix[i]);
            pushed.push(AVAILABLE_CHARS[i]);
            // 先頭から1文字ずつ削り、一致する文字列が状態にあれば、それが移動先の状態
            let mut kk = 0;
            loop {
                kk = prefix.lower_bound(&pushed);
                if kk < k && prefix[kk] == pushed {break}
                pushed = String::from(&pushed[1..]);
            }
            next[i][j] = kk;
        }
    }


    // 動的計画法の初期化
    dp[0][0] = 1; // 長さ0で、それ以外の状態0という文字列""で1個.
    let INF = 1 << 60;
    // 動的計画法



    // 残り文字数のループ
    for t in 0..s.len() {
        // 状態のループ
        for i in 0..k {
            dp[t+1][i] = INF;
        }
        for i in 0..k {
            if ng[i] {continue}
            // 状態iに加える各文字のループ
            for j in 0..AVAILABLE_CHARS.len() {
                let kk = next[i][j];
                dp[t+1][kk] = min(dp[t+1][k], dp[t][i] + s[t] == AVAILABLE_CHARS[j] ? 0 : 1);
            }
        }
    }

    // // dpの中身確認用
    // for i in 0..n+1 {
    //     println!("dp[{}]={:?}", i, dp[i]);
    // }

    let mut ans = INF;
    for i in 0..k {
        if ng[i] {continue}
        ans = min(ans, dp[s.len()][i]);
    }
    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}