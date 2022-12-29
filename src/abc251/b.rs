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
        w: usize,
        a: [usize; n]
    }
    let mut hash_map = HashMap::new();
    // nC3
    // nC2
    // nC1
    for i in 0..n {
        let sum = a[i];
        if sum <= w {
            if !hash_map.contains_key(&sum) {
                hash_map.insert(sum, 1);
            }
        }
        for j in i+1..n {
            if i != j {
                let sum = a[i] + a[j];
                if sum <= w {
                    if !hash_map.contains_key(&sum) {
                        hash_map.insert(sum, 1);
                    }                    
                }
            }
            for k in j+1..n {
                if i != j && j != k && k != i {
                    let sum = a[i] + a[j] + a[k];
                    if sum <= w {
                        if !hash_map.contains_key(&sum) {
                            hash_map.insert(sum, 1);
                        }
                    }
                } 
            }
        }
    }
    println!("{}", hash_map.len());
    // println!("{:?}", hash_map);

}