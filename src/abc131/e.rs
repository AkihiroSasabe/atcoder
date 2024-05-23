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
fn main() {
    // 2024-05-23 19:46-20:36 (50min + 1WA)
    input! {
        n: usize,
        k: usize
    }
    // 最短距離が2である頂点対がK個

    // 存在するなら、グラフを書く
    // 存在しないなら、-1

    let max_k =  (n-1) * (n-2) / 2;
    if k > max_k {
        println!("-1");
        return
    }
    let mut diff = max_k - k;
    let mut edges = vec![];
    for i in 1..n {
        edges.push((0, i));
    }

    for i in 1..n {
        for j in i+1..n {
            if diff == 0 {
                echo_answer(&edges);
                return;
            }
            edges.push((i, j));
            diff -= 1;
        }
    }
    echo_answer(&edges);


}

fn echo_answer(edges: &Vec<(usize, usize)>) {
    println!("{}", edges.len());
    for i in 0..edges.len() {
        println!("{} {}", edges[i].0 + 1, edges[i].1 + 1);
    }
}