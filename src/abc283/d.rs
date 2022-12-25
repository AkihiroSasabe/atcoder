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
        s: Chars
    }
    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut lowermap = HashMap::new();
    for i in 0..lowercase.len() {
        lowermap.insert(lowercase[i], i);
    }
    // println!("{:?}", lowermap);

    let mut flag = true;
    let mut hash_map = HashMap::new();
    let mut left = vec![];
    let INF = 1 << 60;
    let mut before_j = INF;
    for i in 0..s.len() {
        if lowermap.contains_key(&s[i]) {
            if hash_map.contains_key(&s[i]) {
                flag = false;
                break
            }
            else {
                hash_map.insert(&s[i], 1);
            }
        }
        else if s[i] == '(' {
            left.push(i);
        }
        else {
            let j = left.pop().unwrap();
            if before_j == INF || before_j < j {
                for k in j..i {
                    if lowermap.contains_key(&s[k]) {
                        hash_map.remove(&s[k]);
                    }
                }
            }
            else {
                for k in j..before_j {
                    if lowermap.contains_key(&s[k]) {
                        hash_map.remove(&s[k]);
                    }
                }
            }
            before_j = j;
        }
    }

    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}