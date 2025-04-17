#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, thread::panicking};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-04-13 15:42-16:23 (1h41m)
    // 目標 16:12
    // 2025-04-14 20:01-21:15 (1h14m)
    // Total: 2h55min
    // いわゆる、全方位木DPを、自分で再発明して実装していた。 https://ei1333.hateblo.jp/entry/2017/04/10/224413
    // 木DP ... 任意の頂点を根とする、「部分木」についてDPを計算 (遷移が、子達から親へ)
    // 全方位木DP ... 任意の頂点を根とする、 「木」についてDPを計算 (2回dfsを行う。まずは普通に木DP。そのあと、親から子への遷移を考える。)
    input! {
        n1: usize,
    }
    let mut graph1 = vec![vec![]; n1];
    for i in 0..n1-1 {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        graph1[ui].push(vi);
        graph1[vi].push(ui);
    }
    input! {
        n2: usize,
    }
    let mut graph2 = vec![vec![]; n2];
    for i in 0..n2-1 {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        graph2[ui].push(vi);
        graph2[vi].push(ui);
    }


    // max_depths1[v] := 頂点vと頂点vより下に、何層頂点があるか? (頂点0を根とする)
    let mut max_depths1 = vec![0; n1];
    let mut max_depths2 = vec![0; n2];
    dfs_for_max_depths(0, &graph1, &mut max_depths1, 0);
    dfs_for_max_depths(0, &graph2, &mut max_depths2, 0);

    // max_dists1[v] := 頂点vから伸びるエッジのうち、(1番目に遠い頂点の距離, 2番目に遠い頂点の距離)
    let mut max_dists1 = vec![(0,0); n1];
    let mut max_dists2 = vec![(0,0); n2];
    dfs_for_dp(0, &graph1, &max_depths1, &mut max_dists1, 0, 0);
    dfs_for_dp(0, &graph2, &max_depths2, &mut max_dists2, 0, 0);

    let mut diameter1 = 0;
    let mut diameter2 = 0;
    for i in 0..n1 {
        diameter1 = max(diameter1, max_dists1[i].0 + max_dists1[i].1);
    }
    for i in 0..n2 {
        diameter2 = max(diameter2, max_dists2[i].0 + max_dists2[i].1);
    }
    let diamter = max(diameter1, diameter2);

    // println!("max_dists1 = {:?}", max_dists1);
    // println!("max_dists2 = {:?}", max_dists2);

    max_dists2.sort();

    let mut cum2 = vec![];
    for i in 0..n2 {
        cum2.push(max_dists2[i].0);
    }
    for i in 1..n2 {
        cum2[i] = cum2[i] + cum2[i-1];
    }

    let mut ans = 0;
    for i in 0..n1 {
        // max_dists1[i].0 + 1 + max_dists2[j].0 を選ぶか、 diamter を選ぶかって問題
        // つまり、 max_dists2 のうち、 diamter - 1 - max_dists1[i].0 以上のものは、　
        // max_dists1[i].0 + 1 + max_dists2[j].0 を採用し、
        // 未満のものは　diamter　を採用する。

        let ind2 = if diamter - max_dists1[i].0 > 0 {max_dists2.lower_bound(&(diamter - 1 - max_dists1[i].0,0))} else {0};
        let sum = if ind2 != 0 {cum2[n2-1] - cum2[ind2-1]} else {cum2[n2-1]};
        let diff = ind2 * diamter + (n2 - ind2) * (1 + max_dists1[i].0) + sum;

        ans += diff;
    }
    println!("{}", ans);
}


// max_depths[v] := vとvより下に最大何層あるか?
fn dfs_for_max_depths(v: usize, graph: &Vec<Vec<usize>>, max_depths: &mut Vec<usize>, parent: usize) {
    let mut max_depth = 0;
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if nv == parent {continue}
        dfs_for_max_depths(nv, graph, max_depths, v);
        max_depth = max(max_depth, max_depths[nv]);
    }
    max_depths[v] = max_depth + 1;
}

// max_dists[v] := 頂点vから伸びるエッジのうち、(1番目に遠い頂点の距離, 2番目に遠い頂点の距離)
fn dfs_for_dp(v: usize, graph: &Vec<Vec<usize>>, max_depths: &Vec<usize>, max_dists: &mut Vec<(usize, usize)>, parent: usize, pre: usize) {

    let mut lists = vec![];
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if nv != parent {
            lists.push((max_depths[nv], nv));
        }
        else {
            lists.push((pre, nv));
        }
    }
    lists.sort();
    lists.reverse();
    // ダミー
    lists.push((0,0));
    lists.push((0,0));

    for i in 0..graph[v].len() {
        let (dist, nv) = lists[i];
        if nv == parent {continue}
        let mut n_pre = 0;
        if i == 0 {
            n_pre = 1 + lists[1].0;
        }
        else {
            n_pre = 1 + lists[0].0;
        }
        dfs_for_dp(nv, graph, max_depths, max_dists, v, n_pre);
    }
    max_dists[v] = (lists[0].0, lists[1].0);

}