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
        q: usize,
    }
    let mut hakos = vec![BTreeMap::new(); n+1];
    let mut cards = vec![BTreeMap::new(); 200001];
    for i in 0..q {
        input! {
            kind: usize
        }
        if kind == 1 {
            input! {
                i: usize,
                j: usize,
            }
            if hakos[j].contains_key(&i) {
                *hakos[j].get_mut(&i).unwrap() += 1;
            }
            else {
                hakos[j].insert(i, 1);
            }
            cards[i].insert(j, 1);
        }
        else {
            input! {
                i: usize,
            }
            if kind == 2 {
                for (k, v) in hakos[i].iter() {
                    for _ in 0..*v {
                        print!("{} ", k);
                    }
                }
                println!("");
            }
            else {
                for (k, v) in cards[i].iter() {
                    print!("{} ", k);
                }
                println!("");
            }
        }
    }
}