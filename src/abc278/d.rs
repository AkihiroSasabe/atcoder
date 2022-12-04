use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    let INF = 1_000_000_001;
    let mut latest_1 = INF;
    let mut hash_map = HashMap::new();
    for i in 0..a.len() {
        hash_map.insert(i + 1, a[i]);
    }
    for _ in 0..q {
        input! {
            q_i: usize
        }
        if q_i == 1 {
            input! {
                x: usize
            }
            // query.push(vec![q_i, x]);
            latest_1 = x;
            hash_map = HashMap::new();
        } else if q_i == 2 {
            input! {
                i: usize,
                x: usize
            }
            // query.push(vec![q_i, i, x])
            if hash_map.contains_key(&i) {
                *hash_map.get_mut(&i).unwrap() += x;
            } else {
                if latest_1 != INF {
                    hash_map.insert(i, latest_1 + x);
                } else {
                    hash_map.insert(i, x);
                }
            }
        } else {
            input! {
                i: usize
            }
            if hash_map.contains_key(&i) {
                println!("{}", hash_map[&i]);
            } else {
                println!("{}", latest_1);
            }
        }
        // println!("latest1: {}, hash_map: {:?}", latest_1, hash_map);
    }
}
