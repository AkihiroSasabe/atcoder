#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let modulo = 998244353;
        let m_rev = mod_pow(m, modulo - 2, modulo);

    let mut dp = vec![vec![0; n+1]; k];
    for i in 1..(m+1) {
        dp[0][i] += m_rev;
    }
    for kk in 1..k {
        for i in 1..(m+1) {
            for j in 1..(m+1) {
                dp[kk][i] = dp[kk-1][i-j] * 
            }
            // let val = mod_pow(m, modulo - 2, modulo);
        }
    }

    println!("{:?}", dp[0]);


}

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
