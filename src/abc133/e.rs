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
    // 2024-05-18 15:09-15:46 (37min)
    input! {
        n: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            ai: usize,
            bi: usize,
        }
        graph[ai-1].push(bi-1);
        graph[bi-1].push(ai-1);
    } 

    let modulus = 1_000_000_007;

    // 根を決めるのがよいかと。
    // 15:29 ひらめき
    let mut seen = vec![false; n];
    let mut nums = vec![0; n];
    dfs(0, &graph, &mut seen, &mut nums, k, 0);

    let mut ans = 1;
    for i in 0..n {
        ans *= nums[i];
        ans %= modulus;
    }
    println!("{}", ans);

}


fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, nums: &mut Vec<usize>, k: usize, depth: usize) {
    if depth == 0 {
        nums[v] = k;
    }
    let mut num = k-2;
    if depth == 0 {
        num = k - 1;
    }
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        nums[next_v] = num;

        if num == 0 {return}
        num -= 1;
        dfs(next_v, graph, seen, nums, k, depth + 1);
    }
}