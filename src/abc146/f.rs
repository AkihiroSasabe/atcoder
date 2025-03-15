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
    // 2025-03-15 16:34-17:02 (28min)
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }
    // 手数数, 前回出た目
    // let mut inf = 1_000_000_000;
    // dp[i] := i まで達するのに必要な(手数数の最小, 前回からの出目の最小)
    // let mut dp = vec![(inf, inf); n+1];
    // let mut dp = vec![inf; n+1];

    let mut deme = vec![];
    let mut is_bans = vec![false; n+1]; // メモ化再帰で、踏んじゃいけないマスをメモしていくことで、dfsの計算量をO(N)にできる。
    if dfs(n, n, m, &s, &mut deme, &mut is_bans) {
        for ans in deme.iter().rev() {
            print!("{} ", ans);
        }
    }
    else {
        println!("-1");
    }

}

fn dfs(pos: usize, n: usize, m: usize, s: &Vec<char>, deme: &mut Vec<usize>, is_bans: &mut Vec<bool>) -> bool {
    if pos == 0 {return true}
    for step in (1..m+1).rev() {
        if step > pos {continue}
        
        let n_pos = pos - step;
        if s[n_pos] == '1' {continue}
        if is_bans[n_pos] {continue}

        deme.push(step);
        let is_ok = dfs(n_pos, n, m, s, deme, is_bans);
        if is_ok {return true}
        deme.pop();
    }

    is_bans[pos] = true;
    return false
}