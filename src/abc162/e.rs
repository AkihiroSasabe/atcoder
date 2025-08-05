#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-08-01 ?-12:37
    input! {
        n: usize,
        k: usize,
    }
    let modulus = 1_000_000_007;
    
    // Σ[x=1..=k] x*num(gcd==x) になる組み合わせが何個あるかという話。

    // xより大きい互いに素な連中が。

    // n個全部が互いに素な組み合わせは?
    // N以下の素数全列挙?

    // 2,3,7
    // 2,4,7
    // 2^[x], 3^[y], 
    // P個の素数をN個に分ける。
    // P*(p-1)*(p-2)*...

    // gcd は使ってもよい。

    // 1*p
    // 2*p
    // 3*p
    // ...

    // dp[gcd] := gcd のときの、個数
    let mut dp = vec![0; k+1];
    // 競プロフレンズの解説みてupsolve
    for gcd in (1..=k).rev() {
        // k 個
        // [gcd,k] の間に、何個 gcd の倍数があるか?
        let num = k / gcd;
        let cont = get_remainder_for_exp_func(num, n, modulus);
        // println!("gcd = {gcd}, cont = {:?}", cont);
        dp[gcd] = cont;
        for q in 2..=num {
            dp[gcd] = dp[gcd] + modulus - dp[gcd*q];
            dp[gcd] %= modulus;
        }
    }
    // println!("dp = {:?}", dp);
    let mut ans = 0;
    for gcd in 1..=k {
        ans += gcd * dp[gcd];
        ans %= modulus
    }
    println!("{}", ans);



}

// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
fn get_remainder_for_exp_func(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // Example
    // 3^14 mod 100  を求める                                base = 3, exp = 14
    //     3^14  = (3^2)^7                         (mod 100) base = 3^2 = 9, exp = 7
    // <=> 3^14 = (3^2 % 100)^7                    (mod 100)    exp % 2 == 1 => remainder * base
    // <=> 3^14 = (3^4)^3 * (3^2)^1                (mod 100) base = 3^4 = 81, exp = 3
    // <=> 3^14 = (3^4 % 100)^3 * (3^2)^1          (mod 100) 
    // <=> 3^14 = (3^8)^1 * (3^4)^1 * (3^2)^1      (mod 100) base = 3^8 = 6561 = 61, exp = 1
    // <=> 3^14 = (3^8 % 100)^1 * (3^4)^1 * (3^2)^1(mod 100)
    // <=> 3^14 = (6561 % 100)^1 * (81)^1 * (9)^1  (mod 100)
    // <=> 3^14 = (61)^1 * (81)^1 * (9)^1          (mod 100)
    // <=> 3^14 = 44,469 = 69                      (mod 100)
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder;
}

