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
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut inds = vec![0; n];
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
        }
        a.push(ai);
        b.push(bi);
        inds[ai] += 1;
        inds[bi] += 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    for i in 0..n {
        if inds[i] != 2 {
            println!("No");
            return;
        }
    }

    if n != m {
        println!("No");
        return;
    }

    let mut pre = n;
    let mut v = 0;
    for _ in 0..n {
        let mut pre_cand = n;
        let mut v_cand = n;
        for i in 0..2 {
            let nv = graph[v][i];
            if nv == pre {continue}
            pre_cand = v;
            v_cand = nv;
        }
        pre = pre_cand;
        v = v_cand;
        // println!("pre = {:?}", pre);
        // println!("v = {:?}", v);
        if pre == n || v == n {
            println!("No");
            return;
        }
    }
    if v != 0 {
        println!("No");
        return;
    }


    println!("Yes");

}