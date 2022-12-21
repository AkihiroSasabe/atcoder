use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut p = vec![];

    for _ in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
            p_i: isize
        }
        x.push(x_i);
        y.push(y_i);
        p.push(p_i);
    }

    // ダイクストラ法で解く
    // 最短経路問題
    // a->b->c->d
    // ただの幅優先探索で解けそう
    let mut graph = vec![vec![]; n];

    // let mut s = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            let dist = ((x[i] - x[j]).abs() + (y[i] - y[j]).abs());
            println!("i:{} => j: {}, dist:{}, p[i]: {}", i, j, dist, p[i]);
            let mut kuriage = 0;
            if dist % p[i] != 0 {
                kuriage = 1;
            }
            let weight = dist / p[i] + kuriage;
            graph[i].push(vec![j, weight as usize]);
            // s = max(s, dist / p[i] + kuriage);
        }
    }
    for i in 0..n {
        
        graph
    }
    println!("{}", s);
    
    
}