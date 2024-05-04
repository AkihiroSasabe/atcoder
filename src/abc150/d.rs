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
    // 2024-05-03 19:43-20:07 // ぎぶあっぷ (24min)
    // 2024-05-03 20:07-20:40 // はまやんと、競プロフレンズの解説見て、自力で実装してAC (37min)
    // 61min
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }
    // 1 <= X <= M の半公倍数の個数を求める
    // X := a_k * (p + 0.5)
    // <=> X = a_k / 2 * (2p+1)
    // <=> X = b_k * (2p+1) かつ b_k := a_k / 2 とする。
    // Xは、 b_k  (k=0,1,...)の最小公倍数の倍数となる。
    // そして、最小のXとは、b_kの最小公倍数である。

    // 任意のKに対して、成り立たないといけない
    // pは何でもよい。

    // N,M = 2 50
    // a1,a2 = 6 10
    let mut lcm_a = 1;
    let mut lcm_b = 1;
    for i in 0..n {
        lcm_a = lcm(lcm_a, a[i]);
        lcm_b = lcm(lcm_b, a[i] / 2);
    }
    if m < lcm_b {
        println!("0");
    }
    else {
        for i in 0..n {
            if (lcm_b / (a[i] / 2)) % 2 == 0 {
                // 2*p+1 = lcm_b / b_k が奇数でないならアウト。
                println!("0");
                return;
            }
        }
        let ans = (m - lcm_b) / lcm_a + 1;
        println!("{}", ans);
    }

}

fn lcm(x: usize, y: usize) -> usize {
    let g = gcd(x, y);
    let l = x * y / g;
    return l
}

// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}