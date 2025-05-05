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
    // 2025-05-03 21:42-22:15 (33min)
    // 【trit全探索 O(3^N)】N(<=10)個の動物園と、M(<=100)種類の動物がいる。
    // A[x]は、動物xが見れる動物園の集合。c[x]は、動物園xの入園料。
    // M種類の動物を2回ずつ見る最小コストは?
    // 　X: 各動物を「0回観た/1回観た/2回以上観た」の3状態にすると3^100でTLEするが、
    // 　O: 各動物園を「0回訪問/1回訪問/2回訪問」の3状態にすると3^N<=59,049 に収まるので、これを全探索する。
    // 最終的な計算量は、O(3^N * N*M) <= O(6*10^4 * 10^3) = O(6*10^7)で間に合う。
    input! {
        n: usize,
        m: usize,
        c: [usize; n], // 入園料
    }
    let mut k = vec![];
    let mut a = vec![];
    let mut park_to_animals = vec![vec![]; n];
    for i in 0..m {
        input!{
            ki: usize,
            ai: [Usize1; ki], // iがみれる動物園
        }
        k.push(ki);
        a.push(ai.clone());

        for j in 0..ki {
            park_to_animals[ai[j]].push(i);
        }
    }

    let mut pow = vec![1; n+3];
    for i in 1..n+3 {
        pow[i] = 3 * pow[i-1];
    }

    let inf: usize = 1<< 60;
    let mut dp = vec![inf; pow[n]];
    let mut ans = inf;
    // 状態: 59_049 = 6*10^4
    for state in 0..pow[n] {
        let mut num_park = vec![0; n];

        let mut now = state;
        for i in 0..n {
            let r = now % 3;
            num_park[i] = r;
            now /= 3;
        }

        let mut cost = 0;
        let mut num_animal = vec![0; m];
        for i in 0..n {
            cost += num_park[i] * c[i];
            for &ani in park_to_animals[i].iter() {
                num_animal[ani] += num_park[i];
            }
        }
        let mut is_ok = true;
        for i in 0..m {
            if num_animal[i] < 2 {
                is_ok = false;
                break
            }
        }
        if is_ok {
            ans = min(ans, cost);
        }
    }
    println!("{}", ans);
}