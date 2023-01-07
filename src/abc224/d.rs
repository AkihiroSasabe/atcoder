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
    // 2023-01-07 0:09-1:16 分からなかった BFSで解くらしい (1h7m)
    // 2023-01-10 20:08-21:52 解説AC (1h44m)
    // 2h51m必要

    // 取りうる状態数は 9*8*7*6*5*4*3*2 = 362,880 通りしかない
    // 目的の状態は[1,2,3,4,5,6,7,8]
    // 例えば 
    // コマi   1 2 3 4 5 6 7 8
    // 頂点Pi  3 9 2 4 5 6 7 8 (頂点1が空)

    // 頂点1と繋がっているのは、頂点2,3,9
    // したがってここから遷移できる状態は下記の三種類

    // コマi   1 2 3 4 5 6 7 8
    // 頂点Pi  3 9 1 4 5 6 7 8 (頂点2が空)
    // 頂点Pi  1 9 2 4 5 6 7 8 (頂点3が空)
    // 頂点Pi  3 1 2 4 5 6 7 8 (頂点9が空)

    // ==== 添字を0始まりで考察 ====
    // コマ    0 1 2 3 4 5 6 7
    // 頂点Pi  2 8 1 3 4 5 6 7 (頂点0が空: Start)
    // 頂点0と繋がっているのは、頂点1,2,8。頂点0に頂点1,2,8のコマ2,0,1を置く

    // 頂点Pi  2 8 0 3 4 5 6 7 (頂点1が空)
    // 頂点Pi  0 8 1 3 4 5 6 7 (頂点2が空)
    // 頂点Pi  2 0 1 3 4 5 6 7 (頂点8が空)

    // 頂点Pi  0 1 2 3 4 5 6 7 (頂点8が空: Goal)
    
    input! {
        m: usize,
    }
    let mut graph = vec![vec![];9];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[v_i-1].push(u_i-1);
        graph[u_i-1].push(v_i-1);
    }
    input! {
        mut p: [usize;8]
    }
    let mut occupied = vec![false; 9];
    let mut hole_v= 0;
    let mut state = 0;

    for i in 0..8 {
        p[i] -= 1;
        occupied[p[i]] = true;
        state = state * 10;
        state += p[i];
    }
    for i in 0..9 {
        if !occupied[i] {
            hole_v = i;
        }
    }
    let INF = std::usize::MAX;
    let mut distance = vec![INF; 98_765_432+1];
    distance[state] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(vec![state, hole_v]);

    let mut finish_flag = false;

    while queue.len() != 0 {
        let q = queue.pop_front().unwrap();
        let state= q[0];
        let hole_v= q[1];
        // println!("state: {}, hole_v: {}", state, hole_v);
        
        for i in 0..graph[hole_v].len() {
            let next_hole_v = graph[hole_v][i];
            let next_state = get_next_state(state, hole_v, next_hole_v);
            if distance[next_state] != INF {continue}
            distance[next_state] = distance[state] + 1;
            // println!("    next_state: {:08} dist:{}", next_state, distance[next_state]);
            if next_state == 1234567 {
                finish_flag = true;
                break
            }
            queue.push_back(vec![next_state, next_hole_v]);
        }
        if finish_flag {break}
    }
    if distance[1234567] != INF {
        println!("{}", distance[1234567]);
    }
    else {
        println!("{}", -1);
    }
}

fn get_next_state(mut state: usize, hole: usize, next_hole: usize) -> usize {
    let mut scale = 1;
    let mut next_state = state;
    for i in 0..8 {
        let v = state % 10;
        if v == next_hole {
            next_state -= next_hole * scale;
            next_state += hole * scale;
        }
        state /= 10;
        scale *= 10;
    }
    return next_state
}
