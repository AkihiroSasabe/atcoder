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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-03-24 20:29-21:11 (42min)
    input! {
        n: usize,
        ss: Chars,
        x: Chars
    }

    let mut s = vec![];
    for si in ss {
        s.push(si as usize - '0' as usize);
    }
    // 7の倍数なら高橋の勝ち。
    // 高橋、アオキのどちらが勝つか?

    // 0 か si を加える

    // メモ化再帰では?
    // 状態数は、7 * N で済む。

    // i番目より前の数字の剰余。
    let mut seen = vec![vec![(false, false); 7]; n+1];
    let mut pow = vec![1; n+3];
    for i in 1..n+3 {
        pow[i] = (pow[i-1] * 10) % 7;
    }
    if dfs(0, &s, &x, &pow, 0, &mut seen) {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }

}

// 剰余で分けるべき
fn dfs(depth: usize, s: &Vec<usize>, x: &Vec<char>, pow: &Vec<usize>, r: usize, seen: &mut Vec<Vec<(bool, bool)>>) -> bool {
    let n = s.len();
    if depth == s.len() {
        return r % 7 == 0
    }
    if seen[depth][r].0 {
        return seen[depth][r].1
    }

    let res0 = dfs(depth + 1, s, x, pow, r, seen);
    let res1 = dfs(depth + 1, s, x, pow, (r + s[depth] * pow[n - 1 - depth]) % 7, seen);

    if x[depth] == 'T' {
        // 高橋が勝つようにする。
        let is_tkhs_winner = res0 || res1;
        seen[depth][r].0 = true;
        seen[depth][r].1 = is_tkhs_winner;
        return is_tkhs_winner
    }
    else {
        // アオキが勝つようにする。
        let is_aoki_win = !res0 || !res1;
        let is_tkhs_winner = !is_aoki_win;
        seen[depth][r].0 = true;
        seen[depth][r].1 = is_tkhs_winner;
        return is_tkhs_winner
    }
}