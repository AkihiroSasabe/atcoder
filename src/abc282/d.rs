#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    // let mut hash_map = HashMap::new();
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        graph[v_i-1].push(u_i-1);
    }
    // 色
    let mut color = vec![2; n];
    // グラフ彩色 (木のdfsなのでn回ループしなくても、全頂点を探索可能)
    let mut flag = true;
    let mut nibu = vec![];
    let mut non_nibu = vec![];
    for i in 0..n {
        if color[i] != 2 {continue}
        let mut renketu = vec![];
        flag = dfs(&graph, 0, &mut color, 0, &mut renketu);
        if flag {
            nibu.push(renketu);
        }
        else {
            non_nibu.push(renketu)
        }
    }

    // 同色の頂点を格納する(深さが偶数、奇数の頂点で分ける)
    let mut g0 = vec![];
    let mut g1 = vec![];
    for nibu_i in 0..nibu.len() {
        nibu[nibu_i]
    }
    for v in 0..n {
        if color[v] == 0 {g0.push(v)}
        if color[v] == 1 {g1.push(v)}
    }

    let mut ans = 0;
    for v in 0..n {
        let mut count = 0;
        let current_color = color[v];
        for i in 0..graph[v].len() {
            if current_color != color[graph[v][i]] {
                count += 1;
            }
        }
        if current_color == 0 {
            ans += (g1.len() - count);
        }
        else {
            ans += (g0.len() - count);
        }
    }
    ans /= 2;
    if !flag {
        println!("0");
    }
    else {
        println!("{}", ans);
    }


}

// 2部グラフ
fn dfs(graph: &Vec<Vec<usize>>, v: usize, color: &mut Vec<usize>, current_color: usize, renketu: &mut Vec<usize>) -> bool {
    color[v] = current_color;
    renketu.push(v);
    // println!("i: {}, color: {}", v, color[v]);

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if color[next_v] == current_color {return false}
        if color[next_v] == 1 - current_color {continue}
        if !dfs(graph, next_v, color, 1 - current_color, renketu) {return false}
    }

    return true
}
