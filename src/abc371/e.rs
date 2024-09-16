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
use rand::Rng;
fn main() {
    // 2024-09-15 11:50-13:25 (1h35min)
    input! {
        n: usize,
        mut a: [usize; n]
    }
    for i in 0..n {
        a[i] -= 1;
    }

    // Aが全部異なる要素からなる場合の答え ΣΣf(i,j) を取得 (2次元の表で考えると楽)
    let mut ans = 0;
    let mut cum = vec![1; n];
    for i in 1..n {
        cum[i] += cum[i-1];
    }
    for i in 1..n {
        cum[i] += cum[i-1];
    }
    for i in 0..n {
        ans += cum[i];
    }

    // 重複時の寄与を考える。
    // count[x] := (数字 x が登場した回数, 後ろから前へ調べながら、最後に登場したインデックス)
    let mut count = vec![(0, n); n];
    // 後ろから前へ調べる
    for i in (0..n).rev() {
        if count[a[i]].0 > 0 {
            // 重複する寄与を計算
            let height = i + 1;
            let width = n - count[a[i]].1;
            ans -= height * width; // 縦 x 横
            // println!("---------");
            // println!("count[a[i]] = {:?}", count[a[i]]);
            // println!("i = {i}, a[i] = {}, height = {}, width = {:?}", a[i], height, width);
        }

        count[a[i]].0 += 1; // 登場回数を更新
        count[a[i]].1 = i; // 最後に登場時のインデックスを更新
    }
    println!("{}", ans);

}

