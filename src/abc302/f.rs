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
    // 2023-10-05 20:59-22:00 (61min) setの合併方法を調べたりした。
    // 2023-10-05 23:33-24:00 (27min)
    // 2023-10-06 19:19-19:51 (32min) 競プロフレンズの超新星を使った解法で解説AC
    // total: 120min
    input! {
        n: usize,
        m: usize,
    }
    let mut s = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            s_i: [usize; a_i]
        }
        s.push(s_i);
    }

    let s_over = 200_001;
    let mut graph = vec![vec![]; s_over + n];
    for i in 0..n {
        for j in 0..s[i].len() {
            // 超新星(グループ番号)と、数字を繋ぐエッジを作る
            graph[i + s_over].push(s[i][j]);
            graph[s[i][j]].push(i + s_over);
        }
    }
    
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[1] = 0;
    queue.push_back(1_usize);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    // println!("distance = {:?}", distance);

    if distance[m] == init_distance {
        println!("-1");
    }
    else {
        println!("{}", distance[m] / 2 - 1);
    }

}
