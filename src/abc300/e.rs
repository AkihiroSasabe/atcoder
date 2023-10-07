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
    // 2023-10-06 20:34-22:00 (1h26m)
    // 21:08 解法に気づく
    // 2023-10-07 9:20-10:18 (58min)
    // 9:50 w2w2とNyaanの解説を見て、1の目が確率に影響を及ぼさないのがわかる
    // Total (2h24min = 144min)
    input! {
        n: usize
    }

    // dp[n] := nで終わる確率。
    //     dp[n] = (1/6) * (dp[1*n] + dp[2*n] + dp[3*n] + dp[4*n] + dp[5*n] + dp[6*n]);
    // <=> dp[n] = (1/5) * (dp[2*n] + dp[3*n] + dp[4*n] + dp[5*n] + dp[6*n]);
    // <=> これは、サイコロの目の1は結果に影響を及ぼさず、出ないものと考えてよい

    // 計算量の見積もり。N <= 10^18
    // (1,2,3,4,5,6)のうち、素数は(2,3,5)の3つ。
    // そして、取りうる状態数は、
    //    log2(N) * log3(N) * log5(N) 
    // <= log2(10^18) * log3(10^18) * log5(10^18)
    // = 18^3 * (log2(10) * log3(10) * log5(10))
    // < 5832 * (4 * 4 * 2)
    // = 186,624
    // 間に合いそう。

    let modulus = 998244353;
    // Vec<<素数, 指数>>
    let prime_num_list = prime_factorize(n);

    let mut set = HashSet::new();
    for i in 1..7 {
        set.insert(i);
    }
    for i in 0..prime_num_list.len() {
        if !set.contains(&prime_num_list[i][0]) {
            println!("0");
            return;
        }
    }
    let divisors = get_all_divisors(n);
    let mut dp = vec![0; divisors.len()];

    // 状態からindexを取得する
    let mut hash = BTreeMap::new();
    for i in 0..divisors.len() {
        hash.insert(divisors[i], i);
    }

    let inv_5 = mod_inverse(5, modulus);
    dp[0] = 1;
    for i in 0..divisors.len() {
        let state = divisors[i];
        for x in 2..7 {
            let next_state = state * x;
            if !hash.contains_key(&next_state) {continue}
            let next_i = hash[&next_state];
            dp[next_i] += inv_5 * dp[i];
            dp[next_i] %= modulus;
        }
    }
    println!("{}", dp[divisors.len()-1]);

}

/// 自然数nの約数を列挙する
fn get_all_divisors(n: usize) -> Vec<usize> {
    // 素因数分解。prime_num_list := Vec<<素数, 指数>>
    let prime_num_list = prime_factorize(n);
    let mut divisors = vec![];

    let init_prime_index = 0;
    let init_divisor = 1;
    dfs_divisors(init_prime_index, init_divisor, &mut divisors, &prime_num_list);
    divisors.sort();
    return divisors
}

/// 素因数分解[[p0, e0], [p1, e1], ..., [pn, en-1]]の結果から、DFSで全約数を列挙する
/// 約数 := p0^x0 * p1^x1 * p2^x2 * ... (ただし、xi <= ei)
fn dfs_divisors(prime_index: usize, divisor: usize, divisors: &mut Vec<usize>, prime_num_list: &Vec<Vec<usize>>) {
    // prime_index: 注目する素数のindex。注目する素数はprime_num_list[prime_index][0]
    // divisor: 現在の約数
    // divisors: 約数のリスト。これを求める
    // prime_num_list: 素因数分解された結果。Vec<<素数, 指数>>
    
    // 約数を列挙する関数
    // prime_num_listから、約数をすべて列挙する

    if prime_index == prime_num_list.len() {
        divisors.push(divisor);
        return;
    }

    let next_prime_index = prime_index + 1;
    let mut next_divisor = divisor;
    for exp in 0..prime_num_list[prime_index][1]+1 {
        dfs_divisors(next_prime_index, next_divisor, divisors, prime_num_list);
        next_divisor *= prime_num_list[prime_index][0];
    }
}

// 素因数分解
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
    // let root_x = (x as f64).sqrt() as usize;
    let mut prime_num_list = vec![];
    let mut i = 1;
    while i * i <= x {
    // for i in 2..(root_x+1) {
        i += 1;
        let mut exponent = 0;
        while x % i == 0 {
            x /= i;
            exponent += 1;
        }
        if exponent != 0 {
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
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

