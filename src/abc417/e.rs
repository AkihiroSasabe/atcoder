#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-08-02 21:25-21:51 (26min)
    input! {
        t: usize,
    }

    for i in 0..t {
        input! {
            n: usize,
            m: usize,
            x: Usize1,
            y: Usize1,
        }
        let mut graph = vec![vec![]; n];
        for i in 0..m {
            input!{
                ui: Usize1,
                vi: Usize1,
            }
            graph[ui].push(vi);
            graph[vi].push(ui);
        }

        for i in 0..n {
            graph[i].sort();
        }
        dfs(x, &graph, &mut vec![false; n], &mut vec![], y, &mut false);


    }

}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, anss: &mut Vec<usize>, y: usize, is_fin: &mut bool) {
    if *is_fin {return;}
    seen[v] = true;
    anss.push(v);
    if v == y {
        for ans in anss {
            print!("{} ", *ans + 1);
        }
        *is_fin = true;
        println!("");
        return;
    }

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, anss, y, is_fin);
    }
    anss.pop();
}