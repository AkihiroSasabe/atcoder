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
    }
    let mut s = vec![];
    let mut t = vec![];
    for i in 0..n {
        input! {
            s_i: Chars,
            t_i: Chars,
        }
        s.push(s_i);
        t.push(t_i);
    }
    
    // 方針: 有向グラフがサイクルを持つ=>No, 持たない=>Yesを返す
    // 有向グラフのサイクル検出は、BFSによるトポロジカルソート結果の配列数　と　グラフの全頂点数　を比較する
    // ※参考実装: https://ferin-tech.hatenablog.com/entry/2017/01/24/184750
    // ※参考解説: https://algo-logic.info/topological-sort/
    // ※無向グラフの場合は、UnionFindTreeでサイクル検出できる(cf.クラスカル法)
    // ※DFSによるトポロジカルソートはDAGが前提なのでサイクル検出には使えない
    // ※けんちょん氏によるとDFSでサイクル検出する方法もあるようだが、あまりオススメできない https://qiita.com/drken/items/a803d4fc4a727e02f7ba#4-6-%E3%82%B5%E3%82%A4%E3%82%AF%E3%83%AB%E6%A4%9C%E5%87%BA

    // si->ti (i=0,1,...n-1)からなるグラフを格納していく
    let mut graph: HashMap<Vec<char>, Vec<Vec<char>>> = HashMap::new();

    // ユニークな頂点を格納していく
    let mut unique = HashMap::new();

    // 入次数(入ってくる辺の本数)を記録していく
    let mut indegree = HashMap::new();

    // グラフの作成と入次数の計算
    for i in 0..n {
        if graph.contains_key(&s[i]) {
            (*(graph.get_mut(&s[i]).unwrap())).push(t[i].clone());
        }
        else {
            graph.insert(s[i].clone(), vec![t[i].clone()]);
        }

        // 入次数の更新
        if indegree.contains_key(&t[i]) {
            *indegree.get_mut(&t[i]).unwrap() += 1;
        }
        else {
            indegree.insert(t[i].clone(), 1);
        }

        unique.insert(s[i].clone(), 1);
        unique.insert(t[i].clone(), 1);
        
    }

    // トポロジカルソート開始
    // 入次数が0の頂点の集合
    let mut todo = VecDeque::new();

    for i in 0..n {
        if !indegree.contains_key(&s[i]) {
            todo.push_back(s[i].clone());
        }
    }
    // println!("todo: {:?}", todo);

    let mut topological_sorted = vec![];
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        // println!("v: {:?}", v);
        topological_sorted.push(v.clone());

        if !graph.contains_key(&v) {continue}
        for i in 0..graph[&v].len() {
            let next_v = &graph[&v][i];
            // println!("next_v: {:?}", next_v);
            // println!("indegree[next_v]: {:?}", indegree[next_v]);

            *indegree.get_mut(next_v).unwrap() -= 1;
            if indegree[next_v] == 0 {
                todo.push_back(next_v.clone());
            }
        }
    }
    let is_cycle = topological_sorted.len() != unique.len();

    // println!("topological_sorted: {:?}", topological_sorted);
    // println!("unique: {:?}", unique);
    // dbg!(topological_sorted.len());
    // dbg!(unique.len());

    if is_cycle {
        println!("No");
    }
    else {
        println!("Yes");
    }
}
