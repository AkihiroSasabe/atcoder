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
    // 2023-01-25 21:32-21:56 (24min)
    input! {
        l: usize,
        q: usize,
    }
    let mut c = vec![];
    let mut x = vec![];

    for i in 0..q {
        input!{ 
            c_i: usize,
            x_i: usize,
        }
        c.push(c_i);
        x.push(x_i);
    }

    let mut btree = BTreeMap::new();
    btree.insert(0, 'a');
    btree.insert(l, 'a');
    for i in 0..q {
        if c[i] == 1 {
            btree.insert(x[i], 'a');
        }
        else {
            let (k1, _) = btree.range(..x[i]).rev().next().unwrap();
            let (k2, _) = btree.range(x[i]..).next().unwrap();
            // println!("btree:{:?}", btree);
            // println!("x[i]: {}, k1:{}, k2:{}", x[i], k1, k2);
            println!("{}", k2-k1);
        }
    }

}