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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n]
    }

    // 購入総額が安い順に格納されるリスト
    let mut ans = vec![];
    // 購入総額の候補を格納するヒープ
    let mut heap = BinaryHeap::new();
    // 現在の最小購入金額(この額を起点に次の候補を探す)
    let mut current_min = 0;
    // 既に登録済みの購入総額を記録する連想配列
    let mut kakutei = HashMap::new();
    while kakutei.len() < k {
        // ヒープの大きさは最大でNKなので、構築にはO(NKlog(NK))かかっている。
        for i in 0..n {
            // 現時点で一番安い購入総額を起点に、次に高い額の候補を登録していく
            heap.push(-(current_min + a[i]));
        }
        loop {
            let min_cand = -(heap.pop().unwrap());
            if !kakutei.contains_key(&min_cand) {
                current_min = min_cand;
                kakutei.insert(min_cand, 0);
                ans.push(min_cand);
                break
            }
        }
    }
    // println!("{:?}", ans);
    println!("{}", ans[k-1]);

}