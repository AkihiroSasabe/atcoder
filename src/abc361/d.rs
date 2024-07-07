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
    // 2024-07-07 2:11-2:33 (22min) 初提出, TLE
    // 2024-07-07 2:33-3:04 (31min) AC (移動元 が、空洞に引っかかったら、移動できないことに、気付いた)
    // Total: 53 min

    // N <= 14 で少ないので、BFSで状態全探索。
    input! {
        n: usize,
        ss: Chars,
        tt: Chars,
    }
    // 3進法にして解く

    let mut s: u32 = 0;
    let mut t = 0;
    let mut base = 1;
    for i in 0..n {
        if ss[i] == 'B' {
            s += 1 * base;
        }
        else {
            s += 2 * base;
        }
        match tt[i] == 'B' {
            true => t += 1 * base,
            false => t += 2 * base
        }
        base *= 3;
    }
    let mut pow3 = vec![1; n + 3];
    for i in 1..pow3.len() {
        pow3[i] = 3 * pow3[i-1];
    }

    let mut hash: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<(u32, usize, i32)> = VecDeque::new();
    hash.insert(s);
    queue.push_back((s, n, 0)); // 状態, 空洞開始の位置, 必要操作数
    if s == t {
        println!("0");
        return
    }

    // 計算量
    // 状態数: 3^16 = 43_046_721 ~ 4 * 10^7
    // 遷移: 15 -3 = 12
    // 合計: 12 * 3^16 = 516_560_652 ~ 5 * 10^8 // 危険...? 遷移はかけなくて、状態数だけでいいかも。
    while queue.len() != 0 {
        let (state, vaccant, dist) = queue.pop_front().unwrap();
        // i := 移動元
        for i in 0..n+1 {
            // 移動元 が、空洞に引っかかったら、移動できない
            if i == vaccant {continue}
            if i + 1 == vaccant {continue}
            if i == vaccant + 1 {continue}
            let x0: u32 = state / pow3[i] % 3;
            let x1 = state / pow3[i+1] % 3;
            let ns = state + (x0 * pow3[vaccant] + x1 * pow3[vaccant+1]) - (x0 * pow3[i] + x1 * pow3[i+1]);
            if !hash.contains(&ns) {
                if ns == t {
                    println!("{}", dist + 1);
                    return
                }
                queue.push_back((ns, i, dist + 1));
                hash.insert(ns);
            }
        }
    }
    println!("-1");

}