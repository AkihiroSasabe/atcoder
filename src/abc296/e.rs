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
        a: [usize; n]
    }
    let mut graph = vec![vec![]; n];
    let mut reverse_graph = vec![vec![]; n];
    let mut out_degrees = vec![0; n];
    for i in 0..n {
        graph[i].push(a[i]-1);
        reverse_graph[a[i]-1].push(i);
        if i != a[i] - 1 {
            out_degrees[i] += 1;
        }
    }

    // 考察: 
    // ゲームiの高橋の勝ちパターン
    // ・iが閉路である
    // または
    // ・iの出次数が0である
    
    // その頂点がcycleか判定 <=> 
    // dfsの結果、ある頂点を2回通れば

    // 強連結成分分解
    let scc_list = decompositon_of_strongly_connected_components(&graph, &reverse_graph, n);

    let mut ans = 0;
    for i in 0..scc_list.len() {
        // println!("{:?}, out={}", scc_list[i], out_degrees[scc_list[i][0]]);
        
        // 強連結された成分の個数が1
        if scc_list[i].len() == 1 && out_degrees[scc_list[i][0]] == 1 {continue}
        ans += scc_list[i].len();
    }
    println!("{}", ans);


    // 何故かTLEする
    // let mut seen = vec![false; n];
    // let mut cycle = vec![false; n];
    // let mut global_seen = vec![false; n];
    // for v in 0..n {
    //     if cycle[v] || global_seen[v] {continue}
    //     dfs(&graph, v, &mut seen, &mut cycle, &mut global_seen);
    // }

    // let mut ans = 0;
    // for i in 0..n {
    //     if out_degrees[i] == 0 || cycle[i] {
    //         ans += 1;
    //     }
    // }
    // println!("{}", ans);





}


// 1回目のDFS
fn dfs1(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, sorted_list: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs1(graph, *next_v, seen, sorted_list);
    }
    sorted_list.push(v);
}

// 2回目のDFS。トポロジカルソートした番号の逆順から攻める。
fn dfs2(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, scc: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs2(graph, *next_v, seen, scc);
    }
    scc.push(v);
}


// 強連結成分分解 (蟻本p285~p288) 計算量O(E)
fn decompositon_of_strongly_connected_components(graph: &Vec<Vec<usize>>, reverse_graph: &Vec<Vec<usize>>, v_num: usize) -> Vec<Vec<usize>>{

    // 1回目のDFS: トポロジカルソートする
    let mut reverse_topological_sorted_list = vec![];
    let mut seen = vec![false; v_num];
    for v in 0..v_num {
        if seen[v] {continue}
        dfs1(graph, v, &mut seen, &mut reverse_topological_sorted_list);
    }

    // 2回目のDFS: グラフの辺を逆向きにして、たどり着ける頂点を強連結成分としてまとめる
    let mut scc_list = vec![];
    let mut seen = vec![false; v_num];
    reverse_topological_sorted_list.reverse();
    let topological_sorted_list = reverse_topological_sorted_list;
    for v in topological_sorted_list {
        if seen[v] {continue}
        let mut strongly_connected_components = vec![];
        dfs2(&reverse_graph, v, &mut seen, &mut strongly_connected_components);
        scc_list.push(strongly_connected_components);
    }

    return scc_list
}


// 以下はなぜかTLEする
// 全エッジを辿れる
// 入次数が0なら、絶対サイクルじゃない
// 出次数は必ず1
fn dfs(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, cycle: &mut Vec<bool>, global_seen: &mut Vec<bool>) {
    global_seen[v] = true;
    seen[v] = true;
    
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if cycle[next_v] {continue}
        if seen[next_v] {
            cycle[next_v] = true;
        }
        dfs(graph, next_v, seen, cycle, global_seen);
    }
    seen[v] = false;
}



//     N回
// ao: Ki
// ta: 1<=Si<=N

// Ki回繰り返す
// xが書かれている => xを消す => Axを追記
// 最後にiならばtaの勝ち
// 　N回のゲームで高橋の勝つ回数は?

// N
// A1, A2, ...An

// ■入力例１
// 3
// 2,2,3

// 【i=1のゲーム】
// ao:Ki=2
// ta:[1,2,3]
// 最後に1が残るようにしたいけど、、、、
// 1を書く
// 1が書かれている => A1 = 2に変更
// 高橋はこのゲーム、絶対勝てない


// 【i=2のゲーム】
// ta: [1,2,3]
// 2が残るようにしたいけど、
// 2を書く
// 2 => A2 に変更
// 高橋はこのゲーム、絶対勝てる

// 【i=3のゲーム】
// ta: [1,2,3]
// 3が残るようにしたいけど、
// 3を書く
// 高橋はこのゲーム、絶対勝てる

// ■入力例2
// N=2
// A = [2, 1]

// 【i=1のゲーム】
// ao: Ki=1, 2
// ta: if Ki==1 2を指定
// 	else 1を指定


// 3
// [3, 2, 1]


// 仮説：
// Aj == iを満たすjが存在すれば勝てる可能性がある?

// Ki