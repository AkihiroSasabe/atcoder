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
    // 2024-07-06 11:44-12:08 (24min)
    // O(Q N logN) の解法
    input! {
        n: usize,
        m: usize,
        q: usize
    }
    let mut l = vec![];
    let mut r = vec![];
    for i in 0..m {
        input!{
            li: usize,
            ri: usize,
        }
        l.push(li-1);
        r.push(ri-1);
    }
    let mut p = vec![];
    let mut pl = vec![];
    let mut pr = vec![];
    for i in 0..q {
        input!{
            pli: usize,
            pri: usize,
        }
        p.push((pli - 1, pri - 1));
        pl.push(pli-1);
        pr.push(pri-1);
    }
    let mut rails = vec![vec![]; n];
    for i in 0..m {
        rails[l[i]].push(r[i]);
    }
    for i in 0..n {
        rails[i].sort();
    }
    // println!("rails = {:?}", rails);

    for i in 0..q {
        let mut ans = 0;
        for x in pl[i]..pr[i]+1 {
            let ind = rails[x].lower_bound(&(pr[i]+1));
            // println!("x = {x}, ind = {:?}", ind);
            // ans += rails[x].len() - ind;
            // if rails[x].len() == ind {continue}
            ans += ind;
        }
        println!("{}", ans);
    }
}