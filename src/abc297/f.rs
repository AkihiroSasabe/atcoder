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

const MOD: usize = 998244353;


fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
    }
    // let MODULO = 998244353;
    let mut cur = vec![0; w + 1];
    let mut dp = vec![vec![vec![0usize; w + 1]; k + 1]; h + 1];

    for bits in 0..(1usize << (h - 1)) {
        let mut ok = true;
        for i in 0..(h - 2) {
            if (bits >> i) & 1 == 1 && (bits >> (i + 1)) & 1 == 1 {
                ok = false;
            }
        }
        if !ok {
            continue;
        }
        let mut cnt = 0;
        let mut last = 0;
        let mut valid = true;
        for j in 0..w {
            let mut found = false;
            for i in 0..h {
                if i == h - 1 || ((bits >> i) & 1 == 1) {
                    let mut sum = 0;
                    for l in last..i + 1 {
                        sum += cur[l];
                    }
                    if sum > k {
                        valid = false;
                        break;
                    }
                    if last != i {
                        cnt += 1;
                    }
                    last = i + 1;
                    found = true;
                    break;
                }
            }
            if !valid {
                break;
            }
            if found {
                cur = vec![0; w + 1];
                continue;
            }
            for i in 0..h {
                if let Some(bit) = (bits >> i).checked_and(&1) {
                    if bit == 1 {
                        cur[i] += 1;
                    }
                }
            }
        }
        if valid {
            for j in 0..=cnt {
                dp[1][j][cur[j + 1]] += 1;
            }
        }
    }

    for i in 2..=h {
        for j in 0..=k {
            for l in 0..=w {
                for m in 0..=j {
                    dp[i][j][l] += dp[i - 1][m][l] * dp[i - 1][j - m][l] % MOD;
                    dp[i][j][l] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..=k {
        ans += dp[h][i][1];
        ans %= MOD;
    }
    println!("{}", ans);



}

// trait CheckedAnd {
//     fn checked_and(self, other: &Self) -> Option<Self>;
// }

// impl CheckedAnd for usize {
//     fn checked_and(self, other: &Self) -> Option<Self> {
//         if *other == 0 {
//             None
//         } else {
//             Some(self & *other)
//         }
//     }
// }

trait CheckedAnd<T> {
    fn checked_and(self, other: &T) -> Option<Self>;
}

impl CheckedAnd<usize> for usize {
    fn checked_and(self, other: &usize) -> Option<Self> {
        if *other == 0 {
            None
        } else {
            Some(self & *other)
        }
    }
}

trait CheckedAnd: Sized {
    fn checked_and(self, other: &Self) -> Option<Self>;
}

impl CheckedAnd for usize {
    fn checked_and(self, other: &usize) -> Option<Self> {
        if *other == 0 {
            None
        } else {
            Some(self & *other)
        }
    }
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
