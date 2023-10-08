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
    // 2023-10-08 9:43-10:08 (25min, 当日に問題文を読んだのと、測定開始時刻が遅れたので、実際もう少し掛かっている。)
    input! {
        n: usize,
        x: usize,
        t: [usize; n]
    }

    let modulus = 998244353;
    let max_t = 10_001;

    // dp[t][曲番号] = 時刻t+0.5にその曲でいる確率
    let mut dp = vec![vec![0; n]; max_t];
    // dp[t] = 時刻tに新曲を開始出来る確率
    let mut dp_start = vec![0; max_t];
    dp_start[0] = 1;

    let n_inv = mod_inverse(n, modulus as usize) as isize;
    
    // いもす法っぽくやる必要がある。
    for i in 0..n {
        dp[0][i] = n_inv;
        dp[0 + t[i]][i] -= n_inv;
    }

    // dp_startの遷移
    for time in 0..max_t {
        for i in 0..n {
            if time + t[i] >= max_t {continue}
            dp_start[time + t[i]] += dp_start[time] * n_inv;
            dp_start[time + t[i]] %= modulus;
        }
    }

    // dpの遷移
    for time in 1..max_t {
        for i in 0..n {
            // いもす法
            dp[time][i] += dp[time-1][i];
            dp[time][i] %= modulus;
        }
        for i in 0..n {
            dp[time][i] += dp_start[time] * n_inv;
            dp[time][i] %= modulus;
            if time + t[i] >= max_t {continue}
            dp[time + t[i]][i] -= dp_start[time] * n_inv;
            dp[time + t[i]][i] %= modulus;
        }
    }

    println!("{}", ((dp[x][0] % modulus) + modulus) % modulus );


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

