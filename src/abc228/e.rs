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
    // 2024-01-04 12:55-13:44 (49min, 解説ac)
    input! {
        n: usize,
        k: usize,
        m: usize,
    }
    let modulus: usize = 998_244_353;
    
    // 長さn
    // 1 <= a_i <= k
    // 1 <= 数列Anの点数 <= M
    
    // ans = M^(存在しうる数列の組み合わせ)
    // 存在しうる点数の組み合わせ = K^N
    // ans = M^(K^N)

    // フェルマーの小定理より、a^(p-1)=1 (mod p)なので、M^(K^N%(P-1))%Pを求めればいいだけ。
    // K^Nをp-1で割ったあまりを求めればいい。
    // ただし、フェルマーの定理は、aがPの倍数のとき、P^(P-1)=0 (mod P)となって、
    // 成り立たなくなるので注意。場合分けがいる。
    if m % modulus == 0 {
        println!("0");
    }
    else {
        let k_n = mod_pow(k, n, modulus-1);
        let ans = mod_pow(m, k_n, modulus);
        println!("{}", ans);
    }


    // 5^(3^4) != (5^3)^4
    // 証明
    // left = 5^81
    // right = 125 ^ 4 = 5^12

    // 以下ゴミ解法
    // 周期性に着目 (998_244_353 ~ 10^9だからTLEするだけ)
    // let mut hash = HashMap::new();
    // let mut list = vec![];
    // let mut m_pow = 1;
    // loop {
    //     if hash.contains_key(&m_pow) {
    //         break
    //     }
    //     println!("m_pow = {:?}", m_pow);
    //     hash.insert(m_pow, list.len());
    //     list.push(m_pow);
    //     m_pow *= m % modulus;
    //     m_pow %= modulus;
    // }
    // // 周期
    // let t = list.len() - hash.get(&m_pow).unwrap();

    // let mut hash2 = HashMap::new();
    // let mut list2 = vec![];
    // let mut k_pow = 1;
    // loop {
    //     if hash2.contains_key(&k_pow) {
    //         break;
    //     }
    //     println!("k_pow = {:?}", k_pow);
    //     hash2.insert(k_pow, list2.len());
    //     list2.push(k_pow);
    //     k_pow *= k % t;
    //     k_pow %= t;
    // }
    // // 周期2
    // let t2 = list2.len() - hash2.get(&k_pow).unwrap();



}

// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a % modulo * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a % modulo, modulo - 2, modulo)
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

    base %= modulo;
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
