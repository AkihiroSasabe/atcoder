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
    // 2025-01-25 15:37-15:58 (21 min)
    // 2025-02-01 16:20-16:56 (36 min)
    // 2025-02-02 14:42-15:06 (24 min)
    // 2025-02-02 15:06-16:05 (59 min)
    // Total: 140 min
    // 問題の言い換え：以下の条件を満たす十字の中心位置が何通りあるかを数える。
    // 　条件：十字上に、K個の飴がある。
    // 方針：
    // yについて全探索。y=yi上に存在する飴の個数がnum_yのとき、条件を満たす位置xiは、 
    // [1]y=yi上で飴がある点で num_x == k-num_y+1, 
    // [2]y=yi上で飴が無い点で num_x == k-num_y 
    // となるような xi が何個あるかを求めれば良い。
    input! {
        r: usize,
        c: usize,
        k: usize,
        n: usize,
    }
    let mut ys: Vec<Vec<usize>> = vec![vec![]; r];
    let mut xs: Vec<Vec<usize>> = vec![vec![]; c];
    for i in 0..n {
        input!{
            rsi: Usize1,
            csi: Usize1,
        }
        ys[rsi].push(csi);
        xs[csi].push(rsi);
    }

    // num_to_x[個数num] := x=xi上に、要素がnum個あるx
    let mut num_to_x = vec![BTreeSet::new(); n+1];
    for x in 0..c {
        num_to_x[xs[x].len()].insert(x);
    }

    let mut ans = 0;

    // yについて全探索しても良さそう.
    for y in 0..r {
        // println!("y = {:?} --------", y);

        let mut diff = 0;
        let num_y = ys[y].len();
        if num_y > k {continue} // 最初からk超えてたら、スキップ

        let num_x = k - num_y;
        // println!("num_y = {}, num_x = {:?}", num_y, num_x);

        // [1] y＝y 上に、飴がある場所
        for &x in ys[y].iter() {
            // ここでは縦棒内に飴が num_x + 1 個が要求される
            if xs[x].len() == num_x + 1 {
                diff += 1;
            }
        }
        // [2] y=y上に、飴が無い場所
        // ここでは縦棒内に飴が num_x 個が要求される
        diff += num_to_x[num_x].len();
        // ここから、 y=y上で、かつ x上にnum_xある奴を消していく (そこでは縦棒内に飴が、num_x ではなく、 num_x + 1 個必要なため。)
        for &x in ys[y].iter() {
            if num_to_x[num_x].contains(&x) {
                diff -= 1;
            }
        }
        // println!("diff = {:?} ----------", diff);
        ans += diff;
    }
    println!("{}", ans);
}