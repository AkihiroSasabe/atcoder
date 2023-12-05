use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    // 2023-12-05 15:44-16:10 (26min)
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        q: usize,
        xk: [(usize, usize); q]
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let v0 = ab[i].0 - 1;
        let v1 = ab[i].1 - 1;
        graph[v0].push(v1);
        graph[v1].push(v0);
    }
    let mut x = vec![];
    let mut k = vec![];
    for i in 0..q {
        x.push(xk[i].0-1);
        k.push(xk[i].1);
    }
    for i in 0..q {
        // println!("x[i] = {:?}, k[i] = {:?}", x[i], k[i]);
        let sum = bfs(x[i], &graph, k[i]);
        println!("{}", sum);
    }
    // println!("x = {:?}", x);
    // println!("k = {:?}", k);

}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>, max_dist: usize) -> usize {
    // v0が始点
    let mut distance = HashMap::new();
    let mut queue = VecDeque::new();
    distance.insert(v0, 0);
    queue.push_back(v0);
    let mut sum = v0 + 1;
    while queue.len() > 0 {
        // println!("distance = {:?}", distance);
        let v = queue.pop_front().unwrap();
        let dist = *distance.get(&v).unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            // println!("nv = {:?}", nv);
            if distance.contains_key(&nv) {continue}
            if dist + 1 > max_dist {continue}
            sum += nv + 1;
            distance.insert(nv, dist + 1);
            queue.push_back(nv);
        }
    }
    return sum
}