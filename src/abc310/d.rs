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
        t: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    // 相性の悪い人間関係をグラフにする
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..m {
        graph[ab[i].0 - 1].insert(ab[i].1 - 1);
        graph[ab[i].1 - 1].insert(ab[i].0 - 1);
    }

    // teams: 現在存在するチーム
    let mut teams = vec![vec![0]];
    // teamのパターン総数
    let mut ans = 0;
    // 1番目の選手から順に、何番目のチームに入れるかDFSで決めていく
    dfs(0, &graph, &mut teams, &mut ans, t);

    println!("{}", ans);
}

// 1番目の選手から順に、何番目のチームに入れるかDFSで決めていく
// depth:1 [(1)]
// depth:2 [(1, 2)],                       [(1), (2)]
// depth:3 [(1, 2, 3)], [(1, 2), (3)],     [(1, 3), (2)], [(1), (2, 3)], [(1), (2), (3)]
fn dfs(v: usize, graph: &Vec<HashSet<usize>>, teams: &mut Vec<Vec<usize>>, ans: &mut usize, t: usize) {
    // v: 現在注目している選手
    // graph: 相性関係
    // teams: 現在存在するチーム
    // ans: teamのパターン総数
    // t: 規定のteam数

    // 最後の一人か?
    if v == graph.len() - 1 {
        // t個チームが出来たらパターン数を1増加させる
        if teams.len() == t {
            // println!("teams={:?}", teams);
            *ans += 1;
        }
        return
    }

    // v+1をどのチームに所属させるか?
    for i in 0..teams.len() {
        let mut entrance_flag: bool  = true;
        // 相性の悪い選手が居たら入団スキップ
        for j in 0..teams[i].len() {
            let team_member = teams[i][j];
            if graph[v+1].contains(&team_member) {
                entrance_flag = false;
                break
            }
        }
        if !entrance_flag {continue}
        teams[i].push(v+1);
        dfs(v+1, graph, teams, ans, t);
        teams[i].pop();
    }
    // 現在存在しないチームに所属させる
    if teams.len() < t {
        teams.push(vec![v+1]);
        dfs(v+1, graph, teams, ans, t);
        teams.pop();
    }
}
