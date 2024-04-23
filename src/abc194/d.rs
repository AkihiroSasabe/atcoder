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
    // 2024-04-20 18:56-19:23 (27min)
    // 2024-04-20 19:52-?
    // 2024-04-23 19:06-19:43 (37min)
    // 64min
    input! {
        n: usize
    }

    // e[x] := x個の頂点を訪問するのに、必要な操作回数の期待値
    let mut e = vec![0.0; n+1];
    e[1] = 0.0; // 何も操作しなくても、頂点1は既に訪問済み。

    // 遷移
    // e[X] = e[X-1] + Σ[T=1,2,3,...,∞] 回数T * (既に訪問した頂点を、T-1連続で引く確率) * (新規の頂点を訪問できる確率)
    // e[X] = e[X-1] + Σ[T=0,1,2,...,∞] (T+1) * ((X-1)/N)^T * ((N-X+1)/N)
    //      = e[X-1] + ((N-X+1)/N) * Σ[T=0,1,2,...,∞] (T+1) * ((X-1)/N)^T
    // ここで、r<1のとき、
    // S = Σ[T=0,∞](T+1) * r^T = 1/(1-r)^2
    // であるから、最終的な遷移は、
    // e[X] = e[X-1] + N/(N-X+1)
    // となる。

    for i in 2..n+1 {
        e[i] = e[i-1] + n as f64 / (n-i+1) as f64;
    }
    println!("{}", e[n]);

}

