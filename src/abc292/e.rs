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
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    // let mut uft = UnionFindTree::new(n);
    let mut out_edge = vec![0_usize; n];
    let mut in_edge = vec![0_usize; n];
    let mut g_hash = vec![HashMap::new(); n];
    let mut uu = vec![];
    let mut vv = vec![];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        g_hash[u_i-1].insert(v_i-1, 0);
        // uft.unite(u_i-1, v_i-1);
        out_edge[u_i-1] += 1;
        in_edge[v_i-1] += 1;
        uu.push(u_i-1);
        vv.push(v_i-1);
    }
    let mut ans = 0;

    let mut ii = 0;
    while ii < uu.len()  {
        for j in 0..graph[vv[ii]].len() {
            let term = graph[vv[ii]][j];
            if !g_hash[uu[ii]].contains_key(&term) && term != uu[ii] {
                uu.push(uu[ii]);
                vv.push(term);
                ans += 1;
                g_hash[uu[ii]].insert(term, 0);
                graph[uu[ii]].push(term);
                // println!("add: {} -> {}", uu[ii]+1, term+1);
            }
        }
        ii += 1;
    }


}