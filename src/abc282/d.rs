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
    // グラフ彩色 
    let mut is_bipartite = true;
    let mut nibu = vec![];
    let mut non_nibu = vec![];
    // graph全体が連結とは限らないのでforループする必要がある
    for i in 0..n {
        if color[i] != 2 {continue}
        let mut renketu = vec![];
        is_bipartite = dfs(&graph, i, &mut color, 0, &mut renketu);
        // println!("i: {}, is_bipartite: {}",i+1, is_bipartite);
        if is_bipartite {
            nibu.push(renketu);
        }
        else {
            non_nibu.push(renketu)
        }
    }

    // 同色の頂点を格納する(深さが偶数、奇数の頂点で分ける)
    let mut ans = 0;
    for nibu_i in 0..nibu.len() {
        let mut count_in_connected = 0;
        let mut g0 = vec![];
        let mut g1 = vec![];
        for v_i in 0..nibu[nibu_i].len() {
            let v = nibu[nibu_i][v_i];
            if color[v] == 0 {g0.push(v)}
            if color[v] == 1 {g1.push(v)}
        }
        // 同一の連結グラフ内部で条件を満たす辺を数える
        for v_i in 0..nibu[nibu_i].len() {
            let v = nibu[nibu_i][v_i];
            let mut count = 0;
            let current_color = color[v];
            for i in 0..graph[v].len() {
                if current_color != color[graph[v][i]] {
                    count += 1;
                }
            }
            if current_color == 0 {
                count_in_connected += (g1.len() - count);
            }
            else {
                count_in_connected += (g0.len() - count);
            }
        }
        // 異なる連結グラフ同士で条件を満たす辺を数える (2部グラフの異なる連結成分同士は、どの頂点同士でも必ず2部グラフになる。)
        // 注目している連結成分のサイズ * 注目していない連結成分の総サイズ
        count_in_connected += nibu[nibu_i].len() * (n - nibu[nibu_i].len());
        ans += count_in_connected;
    }

    ans /= 2;
    if non_nibu.len() > 0 {
        println!("0");
    }
    else {
        println!("{}", ans);
    }


}

// 2部グラフ判定. 2部グラフならTrueを返し、そうではないならFalseを返す。colorに2色に配色した結果が格納される
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
