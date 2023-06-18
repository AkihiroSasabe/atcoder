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
        w: usize,
        h: usize,
        n: usize,
    }
    let mut p = vec![];
    let mut q = vec![];
    for i in 0..n {
        input! {
            p_i: usize,
            q_i: usize,
        }
        p.push(p_i);
        q.push(q_i);
    }
    input! {
        an: usize,
        mut a: [usize; an],
        bn: usize,
        mut b: [usize; bn]
    }
    a.push(0);
    b.push(0);
    a.sort();
    b.sort();

    // {区画: その区画のいちごの数}を辞書に保存
    let mut hash = HashMap::new();
    for i in 0..n {
        let x_ind = a.lower_bound(&p[i]);
        let y_ind = b.lower_bound(&q[i]);
        if hash.contains_key(&vec![x_ind, y_ind]) {
            *hash.get_mut(&vec![x_ind, y_ind]).unwrap() += 1;
        }
        else {
            hash.insert(vec![x_ind, y_ind], 1);
        }
    }
    let mut max_ans = 0;
    let mut min_ans = n;
    for (k, v) in hash.iter() {
        max_ans = max(max_ans, *v);
        min_ans = min(min_ans, *v);
    }

    // イチゴの存在する区画が、全区画の数より小さい場合、
    // イチゴが存在しない区画がある)
    if hash.len() < (an + 1) * (bn + 1) {
        min_ans = 0;
    }
    println!("{} {}",min_ans, max_ans);

}