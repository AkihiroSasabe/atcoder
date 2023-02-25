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
    // 2023-01-30 12:24-
    // 蟻本p327
    // 文字列を華麗に扱う
    // 問題: k文字の文字列Sを含まないような、n文字の文字列の個数 % MOD を求めよ。
    // ただし、全ての文字列は'A','G','C','T'の4種類から成る。

    let AVAILABLE_CHARS= vec!['A','G','C','T'];
    let MODULO = 10_009;

    // n文字の文字列
    let n = 6;

    // k文字の文字列S
    let k = 4;
    // let s = vec!['A','T','C','G'];
    let s = String::from("ATCG");

    // 1文字加えた際に移動する先の状態
    // 状態0: 下記以外
    // 状態1: **A
    // 状態2: *AT
    // 状態3: ATC
    // 状態4: ATCG (4からの移行先は用意されていない)
    // next[i][j] =: 状態i(=s[0..i])に、AVAILABLE_CHARS[j]を付け加えた際に移動する状態
    // 例えば、状態1(i=1)に、T(j=3)を加えた状態は、next[1][3] = 2 (=*AT) になる.
    let mut next = vec![vec![0; AVAILABLE_CHARS.len()]; k];

    // dp[対象の文字数i][末尾の状態番号j] := 文字数がiのとき、末尾の状態番号jは何個あるか?
    // 例えばdp[2][1]なら、2文字で"*A"なので、AA, TA, CA, GAの4種類
    let mut dp = vec![vec![0; k]; n+1];

    // 前処理
    for i in 0..k {
        for j in 0..AVAILABLE_CHARS.len() {
            // Sと先頭が一致しているi文字
            let sub_str = &s[0..i];

            // Sと先頭が一致しているi文字に1文字加えた文字列
            let mut pushed = String::from(sub_str);
            pushed.push(AVAILABLE_CHARS[j]);
            // println!("i={}, j={}, sub_str: {}, pushed={:?}", i, j, sub_str, pushed);

            // Sの先頭に一致するまで、先頭から1文字削る (最大k回のループ)
            while &s[0..pushed.len()] != pushed {
                pushed = String::from(&pushed[1..]);
                // println!("kezuru: {:?}", pushed);
            }
            // println!("owari: {:?}, next[{}][{}]={}", pushed, i, j, pushed.len());

            // 各状態を示す番号は、状態の長さと紐付けられている。
            // **Aは長さ1, *ATは2, ATCは3, それ以外は0.
            next[i][j] = pushed.len();
        }
    }

    // // nextの中身確認用
    // print!("Add:     ");
    // for i in 0..AVAILABLE_CHARS.len() {
    //     print!("{}  ", AVAILABLE_CHARS[i]);
    // }
    // println!("");
    // for i in 0..k {
    //     println!("next[{}]={:?}, state = {}", i, next[i], &s[0..i]);
    // }

    // 動的計画法の初期状態
    dp[0][0] = 1; // 長さ0で、それ以外の状態0という文字列""で1個.
    // 動的計画法
    // 残り文字数のループ
    for t in 0..n {
        // 状態のループ
        for i in 0..k {
            // 状態iに加える各文字のループ
            for j in 0..AVAILABLE_CHARS.len() {
                let ti = next[i][j];
                if ti == k {continue} // Sが出現してしまうので駄目
                dp[t+1][ti] = (dp[t+1][ti] + dp[t][i]) % MODULO;
            }
        }
    }

    // // dpの中身確認用
    // for i in 0..n+1 {
    //     println!("dp[{}]={:?}", i, dp[i]);
    // }

    let mut ans = 0;
    for i in 0..k {
        ans = (ans + dp[n][i]) % MODULO;
    }
    println!("{}", ans);

}