#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
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
    // 2023-10-13 21:13-22:00 (47min)
    // 2023-10-14 09:09-9:22 (13min, お手上げ)
    // 9:56-10:54 (58min) 物理好きさんの解説を見ながら実装
    // 118min, 1h58min
    input! {
        n: usize,
        a: [usize; n]
    }

    // 線の総数を求める
    let mut sum = 0;

    // dist_list[i] := iのエッジからの最短距離
    let mut dist_list = vec![];
    for i in 0..n {
        // 端からの最短距離
        let dist_from_edge = min(i, n-1-i);
        dist_list.push(dist_from_edge);
    }

    dist_list.sort();
    // println!("dist_list = {:?}", dist_list);
    for i in 0..dist_list.len() - 1 {
        // 連続部分列として登場する回数 * iの相方の個数
        sum += (dist_list[i] + 1) * (dist_list.len() - 1 - i);
    }

    // 良い線の合計本数を求める
    let mut good_sum: usize = 0;

    // num_list[x] := 自然数xが存在するインデックスのリスト
    let mut num_list: Vec<Vec<usize>> = vec![vec![]; a.iter().max().unwrap() + 1];
    for i in 0..n {
        num_list[a[i]].push(i);
    }

    // edge_dist_list[x] := 自然数xが存在するエッジからの距離
    let mut edge_dist_list = vec![vec![]; num_list.len()];
    for i in 0..num_list.len() {
        for j in 0..num_list[i].len() {
            let index = num_list[i][j];
            let edge_dist = min(index, n-1-index);
            edge_dist_list[i].push(edge_dist);
        }
        edge_dist_list[i].sort();
    }

    for i in 0..edge_dist_list.len() {
        if edge_dist_list[i].len() <= 1 {continue}
        for j in 0..(edge_dist_list[i].len()-1) {
            let edge_dist: usize = edge_dist_list[i][j];
            let nokori: usize = edge_dist_list[i].len() - 1 - j;
            // 連続部分列として登場する回数 * jの相方の個数
            good_sum += (edge_dist + 1) * nokori;
        }
    }

    // println!("sum = {:?}", sum);
    // println!("good_sum = {:?}", good_sum);
    // println!("good_sum = {:?}", good_sum);
    // println!("num_list = {:?}", num_list);
    println!("{}", sum - good_sum);

}