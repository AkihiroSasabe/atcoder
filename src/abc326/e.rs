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
    // 2023-10-28 21:13-22:40 (87min) コンテスト中に考察
    // 2023-10-29 09:09-9:54 (45min) わからない...
    // 2023-10-29 09:54-10:37 (43min) 物理好きさんの解説を見ながら実装。
    // total 175min

    // なんとなく、dpっぽいとは思っていたけど、遷移の計算が思いつかなかった。
    // 各時間毎のxでいる確率を求めようと思っていたけど、それは無視してよいことに気づかなかっった。
    // いつでもいいからxにいる確率を求めるという着眼点が抜けていた。
    input! {
        n: usize,
        a: [usize; n]
    }
    let modulus = 998_244_353;
    let inv_n = mod_inverse(n, modulus);

    let mut sum_money = 0;
    for i in 0..n {
        sum_money += a[i];
        sum_money %= modulus;
    }

    // prob_x[i] := x=iになれる確率
    let mut prob_x = vec![0; n];
    let mut prob_x_cum = vec![0; n];
    prob_x[0] = 1;
    prob_x_cum[0] = 1;
    for i in 1..n {
        prob_x[i] = prob_x_cum[i-1] * inv_n % modulus;
        prob_x_cum[i] = (prob_x_cum[i-1] + prob_x[i]) % modulus;
    }
    // println!("prob_x = {:?}", prob_x);
    // println!("prob_x_cum = {:?}", prob_x_cum);

    let mut ans = 0;
    for i in 0..n {
        ans += prob_x[i] * sum_money % modulus * inv_n;
        ans %= modulus;
        // 更新
        sum_money = (modulus + sum_money - a[i]) % modulus;
    }
    println!("{}", ans);

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


