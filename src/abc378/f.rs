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
    // 2024-11-03 17:53-18:15 (22min, 大まかな解法)
    // 2024-11-03 18:15-20:18 (3h3min, debug)
    // 3h25min
    input! {
        n: usize
    }
    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for i in 0..n-1 {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
        degrees[ui-1] += 1;
        degrees[vi-1] += 1;
    }
    let ans = get_subtree_sizes(0, &graph, &degrees);
    println!("{}", ans);
}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>, degrees: &Vec<usize>) -> usize {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut seen = vec![false; n];
    let mut ans = 0;
    dfs(root_v, &graph, &mut seen, degrees, &mut ans);
    return ans
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, degrees: &Vec<usize>, ans: &mut usize) -> usize {
    seen[v] = true;
    let mut num_deg_is_2 = 0; // v より下で、次数が2の頂点の数
    let mut num_deg_is_2_next = 0; // vと直接隣接する頂点で、次数が2の頂点の数
    let mut nums_deg_is_2_for_each_child = vec![]; // vの子毎に、v より下で、次数が2の頂点の数 を格納したリスト
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if seen[nv] {continue}
        let num_deg_is_2_for_child = dfs(nv, graph, seen, degrees, ans);
        num_deg_is_2 += num_deg_is_2_for_child;
        if degrees[nv] == 2 {
            num_deg_is_2_next += 1;
        }
        nums_deg_is_2_for_each_child.push(num_deg_is_2_for_child);
    }

    if v == 0 {
        if degrees[v] == 2 {
            // 自分以外同士の2点を選定 (ただし、同じブランチ同士)
            for num in nums_deg_is_2_for_each_child {
                if num == 0 {continue}
                *ans += num * (num - 1) / 2;
            }

            // 自分と、隣接点以外の2点を選定
            // println!("ans = {:?}", ans);
            *ans += num_deg_is_2 - num_deg_is_2_next;
            return 1
        }
        else if degrees[v] == 3 {
            // 自分以外同士の2点を選定。 n_C_2 で求まる 
            if num_deg_is_2 != 0 {
                let diff = num_deg_is_2 * (num_deg_is_2-1) / 2;
                *ans += diff;
            }
            return num_deg_is_2
        }
        else {
            // 自分以外同士の2点を選定 (ただし、同じブランチ同士)
            for num in nums_deg_is_2_for_each_child {
                if num == 0 {continue}
                *ans += num * (num - 1) / 2;
            }
            return 0
        }
    }

    if degrees[v] == 2 {
        // 自分以外同士の2点を選定 (ただし、同じブランチ同士)
        for num in nums_deg_is_2_for_each_child {
            if num == 0 {continue}
            *ans += num * (num - 1) / 2;
        }

        // 自分と、隣接点以外の2点を選定
        *ans += num_deg_is_2 - num_deg_is_2_next;
        return 1
    }
    else if degrees[v] == 3 {
        return num_deg_is_2
    }
    else {
        // 自分以外同士の2点を選定 (ただし、同じブランチ同士)
        for num in nums_deg_is_2_for_each_child {
            if num == 0 {continue}
            *ans += num * (num - 1) / 2;
        }
        return 0
    }
}