#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
use rand::Rng;
fn main() {
    input! {
        s: Chars
    }

    if s.len() % 2 == 1 {
        println!("No");
        return
    }

    let mut btree = BTreeMap::new();
    for i in 0..s.len()/2 {
        if s[2*i] != s[2*i+1] {
            println!("No");
            // println!("aaaaa");
            return
        }
        *btree.entry(s[2*i]).or_insert(0) += 1;
        *btree.entry(s[2*i+1]).or_insert(0) += 1;
    }

    for (v, num) in btree {
        if num != 2 {
            println!("No");
            // println!("bbbbb");
            // println!("v = {}, num = {:?}", v, num);
            return
        }
    }
    println!("Yes");


}