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
    // 2023-10-21 本番で溶けなかった
    // 2023-10-22 19:00-21:00 (2h = 120min)
    // 2023-10-23 12:11-12:45 (34min) 解説を見た
    // total 154min 
    input! {
        n: usize,

    }
    let mut t = vec![];
    let mut d = vec![];
    
    let mut btree = BTreeMap::new();
    for i in 0..n {
        input! {
            t_i: isize,
            d_i: isize,
        }
        t.push(t_i);
        d.push(d_i);
        btree.entry(t_i).or_insert(vec![]).push(t_i + d_i); // 開始時刻, 終了時刻
    }

    let mut now = 1;
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    loop {
        // 現時刻で印刷出来る奴全員集合
        if btree.contains_key(&now) {
            for end in btree[&now].iter() {
                heap.push(-end);
            }
        }
        
        let mut printed_flag = false;
        while heap.len() > 0 {
            let end = - heap.pop().unwrap();
            if now <= end {
                ans += 1;
                printed_flag = true;
                break
            }
        }

        if printed_flag {
            // 現時点で印字可能なものがあったなら、現時刻を1秒進める
            now += 1;
        }
        else {
            // 現時点で印字可能なものがなかったなら、次に印字可能なものがでてくるまでタイムスリップする
            if let Some((&start, _)) = btree.range(now+1..).next() {
                now = start;
            }
            else {
                // 探索完了
                break
            }
        }
    }

    println!("{}", ans);
}
