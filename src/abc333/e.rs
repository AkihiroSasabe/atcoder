#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::f32::consts::E;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        tx: [(usize, usize); n]
    }
    let mut t = vec![];
    let mut x = vec![];
    for i in 0..n {
        let ti = tx[i].0;
        let xi = tx[i].1;
        t.push(ti);
        x.push(xi - 1);
    }

    let mut x_list = vec![BTreeSet::new(); n];
    for i in 0..n {
        if t[i] == 1 {
            x_list[x[i]].insert(i);
        }
    }

    let mut ans_list = vec![false; n];
    for i in (0..n).rev() {
        // 敵
        if t[i] == 2 {
            let mut ind = n;
            if let Some(k1) = x_list[x[i]].range(..i).rev().next() {
                ind = *k1;
                ans_list[*k1] = true;
            }
            else {
                println!("-1");
                return;
            }
            if ind != n {
                x_list[x[i]].remove(&ind);
            }
        }
    }

    let mut count_temp = 0;
    let mut count_max = 0;
    let mut picks = vec![];
    for i in 0..n {
        // 薬
        if t[i] == 1 {
            if ans_list[i] {
                picks.push(1);
                count_temp += 1;
                count_max = max(count_max, count_temp);
            }
            else {
                picks.push(0);
            }
        }
        // 敵
        else if t[i] == 2 {
            count_temp -= 1;
        }
    }
    println!("{}", count_max);
    for i in 0..picks.len() {
        print!("{} ", picks[i]);
    }


}