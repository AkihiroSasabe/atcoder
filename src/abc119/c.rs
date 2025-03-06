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
    // 2025-03-06 12:37-12:58 (21min)
    // 2025-03-06 19:07-19:13 (6min)
    // 27min
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        l: [usize; n],
    }
    // 長さがa,b,cである竹を作る
    // dpか
    // 全探索か
    //
    // 誰を合成するか?
    // 3つか4つのグループに分ける (Aとして採用,Bとして採用,Cとして採用,不採用)
    // 4^8= 65536 で済みそう.

    // Aに1人,2人,3人,N-2人..
    // Bに1人,2人,...,N-A-1人
    // Cに1人,2人,...N-A-B人

    let mut ans  = 1 << 62;
    dfs(0, &l, n, a, b, c, 0,0,0,0,&mut ans);
    println!("{}", ans - 30); // 最初のmargeには、コストがかからないから。-10 * 3

}

fn dfs(depth: usize, l: &Vec<usize>, n: usize, a: usize, b: usize, c: usize, at: usize, bt: usize, ct: usize, mp: usize, ans: &mut usize) {
    /// 竹A, 竹B, 竹C, 使わない竹 に、各竹を割り振っていく。
    /// at,bt,ctは、マージ後の長さ
    /// そうすると、4 * 4 * ... * 4 = 4^8 = 65536 通りで網羅できる。
    /// 最後の竹まで使ったら、マージ後にどれだけ削るかを決めるだけ。
    if depth == l.len() {
        if at == 0 || bt == 0 || ct == 0 {return}
        
        let aa: usize = (a as isize - at as isize).abs() as usize;
        let bb: usize = (b as isize - bt as isize).abs() as usize;
        let cc: usize = (c as isize - ct as isize).abs() as usize;
        let cost = mp + aa + bb + cc;
        *ans = min(cost, *ans);
        return
    }

    dfs(depth+1, l, n, a, b, c, at + l[depth], bt ,ct, mp + 10, ans);
    dfs(depth+1, l, n, a, b, c, at, bt + l[depth] ,ct, mp + 10, ans);
    dfs(depth+1, l, n, a, b, c, at, bt ,ct + l[depth], mp + 10, ans);
    dfs(depth+1, l, n, a, b, c, at, bt ,ct, mp, ans); // 使わない竹

}