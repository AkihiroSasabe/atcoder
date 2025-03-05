#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
use itertools::Itertools;
use std::{array::IntoIter, cmp::{max, min}};
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
    // 2025-02-16 10:15-10:54 (39min)
    input! {
        n: usize,
        s: Chars
    }

    // i を始点としたときに、距離の合計を求めれば良さそう
    let mut poss = vec![]; // '1'が格納されている位置を格納したリスト
    for i in 0..n {
        if s[i] == '1' {
            poss.push(i);
        }
    }

    // 始点が0のときの、全移動量を求める。
    let mut dist_st0 = 0; // st=0 のときの全移動量
    let mut border = BTreeMap::new(); // <st, 個数> 0になる瞬間のstと、その個数
    let mut num_under0 = 0;
    for i in 0..poss.len() {
        // i番目の'1'を所定の位置まで運んだときの移動量
        let diff = i as isize - poss[i] as isize;
        dist_st0 += diff.abs();

        if diff < 0 {
            *border.entry(diff.abs() as usize).or_insert(0) += 1;
            num_under0 += 1;
        }
    }
    let mut ans = dist_st0 as usize;
    let mut temp = dist_st0 as usize;
    for st in 1..n-poss.len()+1 {
        // 総移動量は、負の数だけ小さくなるし、
        // 0以上の数だけ大きくなる
        let num_0_or_more = poss.len() - num_under0;
        temp = temp - num_under0 + num_0_or_more;

        // 負の数の更新
        if let Some(vvv) = border.get(&st) {
            num_under0 -= *vvv;
        }
        ans = min(ans, temp);
    }
    println!("{}", ans);
}
