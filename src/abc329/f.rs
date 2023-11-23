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
    // 2023-11-19 13:11-13:44 (33min)
    // 解説ac
    input! {
        n: usize,
        q: usize,
        mut c: [usize; n]
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i - 1);
        b.push(b_i - 1);
    }
    let mut hashes = vec![HashSet::new(); n];
    for i in 0..n {
        c[i] -= 1;
        hashes[i].insert(c[i]);
    }
    
    for i in 0..q {
        if hashes[a[i]].len() > hashes[b[i]].len() {
            hashes.swap(a[i], b[i]);
            // swap(&mut hashes[a[i]], &mut hashes[b[i]]);
        }

        let mut temp: HashSet<usize> = HashSet::new();
        swap(&mut temp, &mut hashes[a[i]]);
        // drain は、temp内の要素をiterとして全部放出させて、tempを空にする。
        // extend は、vectorの末尾にiterをpushする
        hashes[b[i]].extend(temp.drain());

        // let mut temp = vec![];
        // for v in hashes[a[i]].iter() {
        //     temp.push(*v);
        // }
        // for v in temp {
        //     hashes[b[i]].insert(v);
        // }
        // hashes[a[i]] = HashSet::new();
        println!("{}", hashes[b[i]].len());
    }

}