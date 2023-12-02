use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    // 2023-12-01 19:57-21:00 (1h3m)
    // 2023-12-02 08:43-10:17 (1h34m) // 公式解説で解説ac
    // 2h37m

    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        graph[v_i-1].push(u_i-1);
    }
    input! {
        q: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..q {
        input! {
            x_i: usize,
            y_i: usize,
        }
        x.push(x_i-1);
        y.push(y_i-1);
    }

    // なもりグラフを、閉路の頂点から生える木に分離
    let group = pseudo_tree::decompose(&graph);
    for i in 0..q {
        if group[x[i]] == group[y[i]] {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }

    // 解説acする前の考察
    // 強連結成分分解を貼るだけでは?
    // 強連結成分は、有向グラフでしか使えない。
    
    // BFSで判定するのがいいかもしれない。
    // 閉路上にあるか、ないか
    // 閉路を超新星とみなしたいが?

    // 同じエッジは二度と使わない

}

/// pseudo-tree := N頂点N辺の連結なグラフ（通称 なもりグラフ。漫画家なもりのツイッターアイコンが名称の由来）
/// ちょうど一つの閉路と、その閉路上の頂点を根とする複数の木として見ることができる。
/// 例: n = 8 個の頂点を持つpseudo-tree
/// 0 -- 1 -- 2
///      |    |
///      3 -- 4 -- 5 -- 6
///                |
///                7
///  閉路:                     頂点1,2,3,4
///  閉路の頂点1を根とする木:   頂点0,1    
///  閉路の頂点4を根とする木:   頂点4,5,6,7
/// 参考: https://atcoder.jp/contests/abc266/editorial/4698 (公式解説)
/// 参考: https://qiita.com/recuraki/items/bb43d0d0f7426f7540e1
mod pseudo_tree {
    /// pseudo-treeを閉路上の頂点を根とする複数の木に分離する。
    pub fn decompose(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        // group[v] := 頂点vの根
        let mut group = vec![graph.len(); graph.len()];
        let is_on_cycle = detect_cycle(graph);
        for v in 0..graph.len() {
            if is_on_cycle[v] {
                dfs(v, &graph, &is_on_cycle, &mut group, v);
            }
        }
        return group
    }

    /// 入次数を調べる
    fn get_in_degree(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let n = graph.len();
        let mut in_degree: Vec<usize> = vec![0; n];
        for v in 0..n {
            in_degree[v] = graph[v].len();
        }
        return in_degree
    }

    /// pseudo-treeの閉路検出
    /// 入次数が1の頂点と、そこから伸びる辺を削除して、次数1の頂点がなくなるまで繰り返す。
    /// 最後に残った頂点がサイクルである
    fn detect_cycle(graph: &Vec<Vec<usize>>) -> Vec<bool> {
        let mut in_degree = get_in_degree(graph);
        let n = graph.len();

        // サイクル上にある頂点を記録
        let mut is_on_cycle = vec![true; n];
        let mut deque = std::collections::VecDeque::new();
        for v in 0..n {
            if in_degree[v] <= 1 {
                // 入次数が1以下のものは、確実に閉路上に存在しない頂点
                deque.push_back(v);
                is_on_cycle[v] = false;
            }
        }
        while deque.len() != 0 {
            let v = deque.pop_front().unwrap();
            for &nv in graph[v].iter() {
                in_degree[nv] -= 1;
                if in_degree[nv] == 1 && is_on_cycle[nv] {
                    deque.push_back(nv);
                    is_on_cycle[nv] = false;
                }
            }
        }
        return is_on_cycle
    }

    /// 深さ優先探索で、頂点 root を根とする木を group に記録していく。
    fn dfs(v: usize, graph: &Vec<Vec<usize>>, is_on_cycle: &Vec<bool>, group: &mut Vec<usize>, root: usize) {
        group[v] = root;
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
    
            // サイクル上の頂点ならスキップ
            if is_on_cycle[nv] {continue}
    
            // 訪問済みの頂点ならスキップ
            if group[nv] != graph.len() {continue}
    
            dfs(nv, graph, is_on_cycle, group, root);
        }
    }
}


