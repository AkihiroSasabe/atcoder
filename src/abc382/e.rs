#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
fn main() {
    // 2024-11-30 21:57-22:40 (43min)
    // 2024-12-01 15:22-15:34 (12min)
    // 2024-12-02 12:30-12:45 (15min)
    // 2024-12-02 18:30-19:54 (84min, 以上が無駄な考察)
    // 2024-12-02 19:55-20:21 (26min, 競プロフレンズの解説見てac)
    // Total: 180min
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }
    // 期待値DPは、メモ化再帰で解く！
    // E(x) := x個のレアカードを手にいれるのに必要な、パック数の期待値
    // prob[num] := パック1個で、レアカードが、 num 枚手に入る確率
    // E(x) = 1個 + (E(x) * prob[0] + E(x-1) * prob[1] + E(x-2) * prob[2] + ... + E(x-n) * prob[n])
    // E(x) = (1個 + E(x-1) * prob[1] + E(x-2) * prob[2] + ... + E(x-n) * prob[n]) / (1 - prob[0])
    // 
    // 例
    // E(0) = 0
    // E(1) = 1 + E(1) * prob[0] + E(0) * prob[1]
    // E(2) = 1 + E(2) * prob[0] + E(1) * prob[1] + E(0) * prob[2]
    // E(3) = 1 + E(3) * prob[0] + E(2) * prob[1] + E(1) * prob[2] + E(0) + prob[3]

    // 5000 * 5000 = 2.5 * 10^7
    // prob[num] := パック1個で、レアカードが、 num 枚手に入る確率
    let mut prob = vec![0.0; n+2];
    prob[0] = 1.0; // 0パックなら、0枚で100%。
    for i in 0..n {
        let mut pre_prob = vec![0.0; n+2];
        swap(&mut prob, &mut pre_prob);

        for num in 0..n+1 {
            let pi = p[i] as f64 / 100.0; // i 枚目がレアな確率
            prob[num] += pre_prob[num] * (1.0 - pi);
            prob[num+1] += pre_prob[num] * pi;
        }
    }
    // println!("prob = {:?}", prob);

    // 期待値 expectations[num] := num 枚のレアカードを取得するのに、必要なパック数の期待値
    let mut expectations: Vec<f64> = vec![0.0; x + 1];
    for xi in 1..x+1 {
        let mut top = 1.0;
        for diff in 1..n+1 {
            if xi < diff {continue}
            top += expectations[xi-diff] * prob[diff];
        }
        let bottom = 1.0 - prob[0];
        expectations[xi] = top / bottom;
    }
    println!("{}", expectations[x]);
}

