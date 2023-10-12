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
    // 2023-10-12 19:26-
    // 20:15 思いついた (+49min)
    // 21:05 初提出 WA (+50min)
    // 21:10 デバッグAC (+5min)
    // total (1h44m = 104m)
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '1' {
                let next_v = i + 1 + j;
                if next_v >= n {continue}
                graph[i].push(next_v);
                rev_graph[next_v].push(i);
            }
        }
    }

    let distance_from_start = bfs(0, &graph);
    let distance_from_goal = bfs(n-1, &rev_graph);
    // println!("distance_from_start = {:?}", distance_from_start);
    // println!("distance_from_goal = {:?}", distance_from_goal);

    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    // 連結
    for i in 1..(n-1) {
        // println!("k={}, i={}---------------", i+1, i);
        // そもそもs-gパスが存在しないとき
        if distance_from_start[n-1] == init_distance {
            print!("-1 ");
            // println!("aaaa");
            continue
        }

        if distance_from_start[i] != init_distance && distance_from_goal[i] != init_distance {
            // iがs-gパス上に居るとき

            // iがいない場合の経路
            // iより前のm個のどれかを通るはず。
            let mut ans = init_distance;
            for j in 1..m+1 {
                if i < j {continue}
                let pre = i - j;
                for x in 0..graph[pre].len() {
                    let pre_next = graph[pre][x];
                    if pre_next <= i {continue}
                    let dist = distance_from_start[pre] + 1 + distance_from_goal[pre_next];
                    // println!("pre = {}, pre_next={}, dist={dist}, distance_from_start[pre_next]={}, ", pre+1, pre_next+1, distance_from_start[pre_next]);
                    ans = min(ans, dist);
                }
            }
            if ans == init_distance {
                print!("-1 ");
                // println!("bbbb");
            }
            else {
                print!("{} ", ans);
                // println!("cccc");
            }
            
        }
        else {
            // iがs-gパス上に居ないとき
            // 何も影響が無い
            print!("{} ", distance_from_start[n-1]);
            // println!("dddd");
        }
    }
}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[v0] = 0;
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    return distance
}