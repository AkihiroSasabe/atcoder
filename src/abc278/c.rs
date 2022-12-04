#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
    let mut t = vec![];
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input! {
            t_i: usize,
            a_i: usize,
            b_i: usize,
        }
        t.push(t_i);
        a.push(a_i - 1);
        b.push(b_i - 1);
    }
    let mut follower: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for i in 0..q {
        if t[i] == 1 {
            if follower.contains_key(&b[i]) {
                if follower[&b[i]].is_empty() {
                    let mut inner_hash_map = HashMap::new();
                    inner_hash_map.insert(a[i], 1);
                    follower.insert(b[i], inner_hash_map);
                } else {
                    (*follower.get_mut(&b[i]).unwrap()).insert(a[i], 1);
                }
            } else {
                let mut inner_hash_map = HashMap::new();
                inner_hash_map.insert(a[i], 1);
                follower.insert(b[i], inner_hash_map);
                // entry().or_insert(inner_hash_map);
            }
        } else if t[i] == 2 {
            if follower.contains_key(&b[i]) {
                (*follower.get_mut(&b[i]).unwrap()).remove(&a[i]);
            }
        } else if t[i] == 3 {
            if follower.contains_key(&a[i])
                && follower[&a[i]].contains_key(&b[i])
                && follower.contains_key(&b[i])
                && follower[&b[i]].contains_key(&a[i])
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}