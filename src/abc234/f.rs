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
    // 2023-12-20 19:42-22:17 (2h35m) 競プロフレンズの解説見てACした
    input! {
        s: Chars
    }
    // dpでいけそう。
    const MODULUS: usize = 998244353;
    let n = s.len();

    let mut kaijo = vec![];
    let mut inv_kaijo = vec![];
    let mut temp = 1;
    let mut inv_temp = 1;
    kaijo.push(temp);
    inv_kaijo.push(inv_temp);
    for i in 1..5001 {
        temp *= i;
        temp %= MODULUS;
        kaijo.push(temp);

        inv_temp *= mod_inverse(i, MODULUS);
        inv_temp %= MODULUS;
        inv_kaijo.push(inv_temp);
    }

    let mut counter = vec![0; 26];
    for i in 0..n {
        counter[s[i] as usize - 'a' as usize] += 1;
    }

    // dp[何番目のアルファベットまで使ったか?][合計何文字使ったか]
    // dp[i][j] := 
    // 例: a,b,cだけで作れる長さ5の文字列で、cを2文字目と5文字目に使うもの
    // acbbc -> abb と cc に分離可能。
    // より一般化すると、
    // dp[2][5] := aとbとcで、5文字作る組み合わせ数
    // dp[2][5] = dp[1][5] * 5C0 <=> aとbで5文字作る組み合わせ数 * 5文字の中からcを0個選ぶ: _____ 
    //          + dp[1][4] * 5C1 <=> aとbで4文字作る組み合わせ数 * 5文字の中からcを1個選ぶ: ____c
    //          + dp[1][3] * 5C2 <=> aとbで3文字作る組み合わせ数 * 5文字の中からcを2個選ぶ: ___cc
    //          + dp[1][2] * 5C3 <=> aとbで2文字作る組み合わせ数 * 5文字の中からcを3個選ぶ: __ccc
    //          + dp[1][1] * 5C4 <=> aとbで1文字作る組み合わせ数 * 5文字の中からcを4個選ぶ: _cccc
    //          + dp[1][0] * 5C5 <=> aとbで0文字作る組み合わせ数 * 5文字の中からcを5個選ぶ: ccccc
    // 
    // dp[i][j] = ∑[k=0, j] dp[i-1][j-k] * jCk // i-1番目の文字をk個使う。

    let mut dp = vec![vec![0; n+1]; 26];
    for i in 0..counter[0]+1 {
        dp[0][i] = 1;
    }

    // println!("dp[0] = {:?}", dp[0]);

    // dp[1][0] = 1
    // dp[1][1] = 2 (a, b)
    // dp[1][2] = 3 (aa, ab, ba)

    for i in 1..26 {
        // i:= 使っていいindex
        for j in 0..n+1 {
            // j := 長さ
            for k in 0..min(j, counter[i])+1 {
                // k := i番目のアルファベットの個数
                let jck = bi_nominal(j, k, &kaijo, &inv_kaijo, MODULUS);
                // println!("{j}c{k} = {:?}", jck);
                dp[i][j] += dp[i-1][j-k] * jck % MODULUS;
                dp[i][j] %= MODULUS;
            }
        }
        // println!("dp[{i}] = {:?}", dp[i]);
    }

    let mut ans = 0;
    for j in 1..n+1 {
        ans = (ans + dp[25][j]) % MODULUS;
    }
    println!("{}", ans);

}

fn bi_nominal(n: usize, r: usize, kaijo: &Vec<usize>, inv_kaijo: &Vec<usize>, modulus: usize) -> usize {
    kaijo[n] * inv_kaijo[r] % modulus * inv_kaijo[n-r] % modulus
}


// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}
