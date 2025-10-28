#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-10-26 11:02-11:47 (45min)
    // 2025-10-25 コンテスト中、C問題解答後に、D問題とE問題を同時に考察していたので、実際は45分以上かかっている。
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        graph[ui].push(vi);
        graph[vi].push(ui);
    }
    input! {
        ss: Chars
    }

    let mut oks = vec![];
    let mut ngs = vec![];
    for i in 0..n {
        if ss[i] == 'S' {
            oks.push(i);
        }
        else {
            ngs.push(i);
        }
    }

    // 方針: dp[v] に安全頂点が2個まで入るように、安全頂点から多始点BFSを行う。
    // dp[v] := <vに2番目以内に到達した安全頂点, その安全頂点とvとの距離>
    let mut dp = vec![BTreeMap::new(); n];

    // queue := (頂点, 到達した安全頂点, その安全頂点と頂点との距離) を格納
    let mut queue = VecDeque::new();
    for &ok_v in oks.iter() {
        dp[ok_v].insert(ok_v, 0_usize);
        queue.push_back((ok_v, ok_v, 0_usize));
    }
    while queue.len() > 0 {
        let (v, ok_v, dist) = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if dp[nv].contains_key(&ok_v) {continue}
            if dp[nv].len() >= 2 {continue}
            dp[nv].insert(ok_v, dist + 1);
            queue.push_back((nv, ok_v, dist + 1));
        }
    }
    for ng_v in ngs {
        let mut ans = 0;
        for (ok_v, dist) in dp[ng_v].iter() {
            ans += dist;
        }
        println!("{}", ans);
    }
}
