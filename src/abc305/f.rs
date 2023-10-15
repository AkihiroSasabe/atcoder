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
use std::collections::BTreeSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-09-29 12:12-12:50 (38min)
    // 2023-09-29 20:19-21:00 (41min)
    // 2023-09-30 10:00-13:32 (3h32min = 212min) HeapとかB木でゴチャゴチャ頑張ったけど、結局DFS木で解説ACした
    // total (291min=4h51min)
    let inputs = read_multi();
    let n = inputs[0];
    let m = inputs[1];

    solve(n, m);
}

fn solve(n: usize, m: usize) {
    // 方針
    // 1度訪れた頂点には進まず、引き返す
    // いわゆるDFS木になるので、2N-2回で済む

    let mut stack = vec![];
    let mut seen = vec![false; n];
    // let mut is_back_mode = false;
    
    // 質問2N回
    let mut v = 0;
    for i in 0..2*n+10 {
        let inputs = read_multi();
        let k = inputs[0];

        stack.push(v);
        seen[v] = true;
        
        let mut is_dead_end = true; // vが袋小路か?
        let mut next_v = n; // 適当に初期化
        for j in 1..k+1 {
            let next_v_temp = inputs[j] - 1;
            if !seen[next_v_temp] {
                is_dead_end = false;
                next_v = next_v_temp;
                // is_back_mode = false;
                break;
            }
        }
        
        if is_dead_end {
            stack.pop(); // vを取り出す
            next_v = stack.pop().unwrap(); // vの1個前
            // is_back_mode = true;
        }

        println!("{}", next_v + 1);

        // 探索する頂点を更新
        v = next_v;

        // 頂点Nへ移動できたら終了
        if next_v == n - 1 {
            let result = read_string();
            // println!("result = {:?}", result);
            if result == "OK" {return}
        }
    }
}

fn solve_ng4(n: usize, m: usize) {
    // 訪れた頂点数が少ない頂点に貪欲に進んでいく
    // このやり方だと、同じ頂点を4回通ってしまうことが、あるのでNG
    let mut graph: Vec<BTreeSet<Vec<usize>>> = vec![BTreeSet::new(); n];
    let mut seen_count: Vec<usize> = vec![0; n];
    
    // 質問2N回
    let mut v = 0;
    for i in 0..2*n+10 {
        seen_count[v] += 1;
        // println!("i = {i}, v = {v}");
        let inputs = read_multi();
        let k = inputs[0];
        
        graph[v] = BTreeSet::new();
        for j in 1..k+1 {
            let next_v = inputs[j] - 1;
            let count = seen_count[next_v];
            let edge = vec![count, next_v];
            graph[v].insert(edge);
        }

        // 逆流も込みで考えてほしい。 -> Heapは駄目。B木でやるべき
        // 逆辺の情報もアクセス出来るようなグラフが要るな。

        // (1-1)グラフの順方向を更新
        let edge_state = (*graph[v].iter().next().unwrap()).clone();
        let next_v = edge_state[1];

        // println!("v={}, next_v={}", v, next_v);
        // println!("graph[v] = {:?}", graph[v]);
        // println!("graph[next_v] = {:?}", graph[next_v]);
        // println!("ref_graph[v] = {:?}", ref_graph[v]);
        // println!("ref_graph[next_v] = {:?}", ref_graph[next_v]);

        println!("{}", next_v + 1);

        // 探索する頂点を更新
        v = next_v;

        // 頂点Nへ移動できたら終了
        if next_v == n - 1 {
            let result = read_string();
            // println!("result = {:?}", result);
            if result == "OK" {return}
        }
    }
}





fn solve_ng3_btree(n: usize, m: usize) {
    let mut graph: Vec<BTreeSet<Vec<usize>>> = vec![BTreeSet::new(); n];
    let mut ref_graph: Vec<HashMap<usize, Vec<usize>>> = vec![HashMap::new(); n];
    let mut seen_edge = vec![HashSet::new(); n];
    let mut seen_count = vec![0; n];
    
    let COUNT_INIT: usize = 0;

    // 質問2N回
    let mut v = 0;
    for i in 0..2*n+10 {
        seen_count[v] += 1;
        // println!("i = {i}, v = {v}");
        let inputs = read_multi();
        let k = inputs[0];

        // 初見の頂点であれば、エッジを構築していく
        for j in 1..k+1 {
            // そのエッジに与えられたHPと、そのエッジの行き先の頂点をHeapに格納
            // HPは一回通る度に減っていく
            let next_v = inputs[j] - 1;
            if seen_edge[v].contains(&next_v) {continue}
            seen_edge[v].insert(next_v);
            seen_edge[next_v].insert(v);

            let edge = vec![COUNT_INIT, COUNT_INIT, next_v];
            let rev_edge = vec![COUNT_INIT, COUNT_INIT, v];
            graph[v].insert(edge.clone());
            graph[next_v].insert(rev_edge.clone());

            // println!("initi graph[{v}] = {:?}", graph[v]);
            ref_graph[v].insert(next_v, edge);
            ref_graph[next_v].insert(v, rev_edge);
        }

        // 逆流も込みで考えてほしい。 -> Heapは駄目。B木でやるべき
        // 逆辺の情報もアクセス出来るようなグラフが要るな。

        // (1-1)グラフの順方向を更新
        let edge_state = (*graph[v].iter().next().unwrap()).clone();
        let count_forward = edge_state[0];
        let count_backward = edge_state[1];
        let next_v = edge_state[2];

        // println!("v={}, next_v={}", v, next_v);
        // println!("graph[v] = {:?}", graph[v]);
        // println!("graph[next_v] = {:?}", graph[next_v]);
        // println!("ref_graph[v] = {:?}", ref_graph[v]);
        // println!("ref_graph[next_v] = {:?}", ref_graph[next_v]);

        // エッジ情報を更新していく
        let updated_edge_state = vec![count_forward + 1, count_backward, next_v];
        graph[v].remove(&edge_state);
        graph[v].insert(updated_edge_state);

        // (1-2)グラフの逆方向を更新
        // 逆辺の情報も更新していく
        let rev_edge_state = ref_graph[next_v].get(&v).unwrap();
        graph[next_v].remove(rev_edge_state);
        let updated_rev_edge_state = vec![rev_edge_state[0], rev_edge_state[1] + 1, rev_edge_state[2]];
        graph[next_v].insert(updated_rev_edge_state);

        // (2-1)refグラフの逆方向を更新
        ref_graph[next_v].get_mut(&v).unwrap()[1] += 1;
        // (2-2)refグラフの順方向を更新
        ref_graph[v].get_mut(&next_v).unwrap()[0] += 1;


        // println!("graph[{v}] = {:?}", graph[v]);
        
        println!("{}", next_v + 1);

        // 探索する頂点を更新
        v = next_v;

        // 頂点Nへ移動できたら終了
        if next_v == n - 1 {
            let result = read_string();
            // println!("result = {:?}", result);
            if result == "OK" {return}
        }
    }
}


fn solve_ng2_heap(n: usize, m: usize) {
    let mut graph: Vec<BinaryHeap<Vec<usize>>> = vec![BinaryHeap::new(); n];
    let mut seen = vec![false; n];
    let COUNT_INIT: usize = 1_000_000_000_000_000;

    // 質問2N回
    let mut v = 0;
    for i in 0..COUNT_INIT {
        // println!("i = {i}, v = {v}");
        let inputs = read_multi();
        let k = inputs[0];

        // 初見の頂点であれば、エッジを構築していく
        if !seen[v] {
            for j in 1..k+1 {
                // そのエッジに与えられたHPと、そのエッジの行き先の頂点をHeapに格納
                // HPは一回通る度に減っていく
                graph[v].push(vec![COUNT_INIT, COUNT_INIT, inputs[j] - 1]);
                // println!("initi graph[{v}] = {:?}", graph[v]);
            }
        }
        else {
            seen[v] = true;
        }

        // 逆流も込みで考えてほしい。 -> Heapは駄目。B木でやるべき
        // 逆辺の情報もアクセス出来るようなグラフが要るな。
        let next_state = graph[v].pop().unwrap();
        let count_forward = next_state[0];
        let count_backward = next_state[1];
        let next_v = next_state[2];
        // println!("count={count}. next_v={next_v}");
        graph[v].push(vec![count_forward - 1, count_backward, next_v]);


        // println!("graph[{v}] = {:?}", graph[v]);
        
        println!("{}", next_v + 1);

        v = next_v;

        // 頂点Nへ移動できたら終了
        if next_v == n - 1 {
            let result = read_string();
            // println!("result = {:?}", result);
            if result == "OK" {return}
        }
    }
}

fn solve_ng_1(n: usize, m: usize) {
    // let mut seen = vec![false; n];
    let mut seen_hash = vec![HashSet::new(); n];

    let mut next_indice = vec![1; n];

    // 質問2N回
    let mut v = 0;
    for i in 0..2*n {
        println!("i = {i}, v = {v}");
        let inputs = read_multi();
        let k = inputs[0];

        let mut next_index = next_indice[v];

        let mut detect_flag = false;

        for j in next_index..inputs.len() {
            next_indice[v] = j + 1;
            let next_v = inputs[j] - 1;
            if seen_hash[v].contains(&next_v) {continue}
            if seen_hash[next_v].contains(&v) {continue}
            seen_hash[v].insert(next_v);
            seen_hash[next_v].insert(v);

            // if seen[next_v] {continue}
            // seen[next_v] = true;
            println!("{}", next_v + 1);
            detect_flag = true;

            v = next_v;

            // 頂点Nへ移動できたら終了
            if next_v == n - 1 {
                let result = read_string();
                // println!("result = {:?}", result);
                if result == "OK" {return}
            }
            else {
                break
            }
        }

        if !detect_flag {
            println!("{}", v + 1);
        }
    }
}

// インタラクティブな読み込みをする関数 (1行に多変数)
fn read_multi() -> Vec<usize> {
    // println!("input multi!!!!!!!");
    let mut line_string = String::new();
    std::io::stdin().read_line(&mut line_string).expect("入力エラー");
    let line_str_list: Vec<&str> = line_string.split_whitespace().collect();
    let line_usize_list: Vec<usize> = line_str_list.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    return line_usize_list
}

// 文字列のインタラクティブな読み込みをする関数 (1行に1変数)
fn read_string() -> String {
    // println!("input string!!!!!!!");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    // trim()は末尾の"\n"を消して、&str型で返す。(trimしないとString型のまま)
    return s.trim().to_string()
    // s.trim().parse().unwrap()
}