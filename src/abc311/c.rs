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
        a: [usize; n]
    }
    let mut graph = vec![vec![]; n];
    let mut ind = vec![0; n];
    for i in 0..n {
        graph[i].push(a[i]-1);
        ind[a[i]-1] += 1;
    }

    let mut seen = vec![false; n];
    let mut circle = vec![false; n];
    let mut circle_v = vec![];
    let mut origin = n;
    for v in 0..n {
        if seen[v] {continue}
        // println!("---- start dfs v = {} ----", v+1);
        dfs(v, &graph, &mut seen, &mut circle, &mut circle_v, &mut origin);
        // println!("circle_v = {:?}", circle_v);
        if origin != n {
            let mut flag = false;
            for i in 0..circle_v.len() {
                if !flag {
                    if origin != circle_v[i] {
                        continue
                    }
                    else{
                        println!("{}", circle_v.len() - i);
                        flag = true;
                    } 
                }
                print!("{} ", circle_v[i] + 1);
            }
            return
        }
    }

}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, circle: &mut Vec<bool>, circle_v: &mut Vec<usize>, origin: &mut usize) -> bool {
    // println!("in v={}", v + 1);
    seen[v] = true;
    circle[v] = true;
    circle_v.push(v);

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if circle[next_v] {
            *origin = next_v;
            return false}
        if seen[next_v] {continue}
        if !dfs(next_v, graph, seen, circle, circle_v, origin) {return false}
    }
    circle_v.pop();
    // println!("out v={}", v + 1);
    circle[v] = false;
    return false

}

