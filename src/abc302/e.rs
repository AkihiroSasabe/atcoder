#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        q: usize,

    }
    let mut ans = n;
    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    let mut cin = vec![0; n];
    for i in 0..q {
        input! {
            kind: usize,
            u: usize,
        }
        if kind == 1 {
            // u-v結合
            input! {
                v: usize
            }
            cin[u-1] += 1;
            cin[v-1] += 1;
            if cin[u-1] == 1 {
                ans -= 1;
            }
            if cin[v-1] == 1 {
                ans -= 1;
            }
            graph[u-1].insert(v-1, 0);
            graph[v-1].insert(u-1, 0);
            println!("{}", ans);
        }
        else if kind == 2 {
            // uと連結した辺を削除
            let mut erase_list = vec![];
            for (v, _) in graph[u-1].iter() {
                // let v = graph[u-1][i];
                cin[u-1] -= 1;
                cin[*v] -= 1;
                if cin[u-1] == 0 {
                    ans += 1;
                }
                if cin[*v] == 0 {
                    ans += 1;
                }
                erase_list.push(*v);
            }
            for i in 0..erase_list.len() {
                let v = erase_list[i];
                graph[u-1].remove(&v);
                graph[v].remove(&(u-1));
            }
            println!("{}", ans);
        }
    }
}
