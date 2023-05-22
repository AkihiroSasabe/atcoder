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
        S: Chars,
        T: Chars
    }
    
    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut hash = HashMap::new();
    for i in 0..lowercase.len() {
        hash.insert(lowercase[i], i);
    }
    let mut moji_num_s = vec![0 as isize; lowercase.len()];
    let mut moji_num_t = vec![0 as isize; lowercase.len()];
    let mut at_num_s = 0;
    let mut at_num_t = 0;
    for i in 0..S.len() {
        if S[i] == '@' {
            at_num_s += 1;
        }
        else {
            moji_num_s[hash[&S[i]]] += 1;
        }
        if T[i] == '@' {
            at_num_t += 1;
        }
        else {
            moji_num_t[hash[&T[i]]] += 1;
        }
    }
    
    let mut diff = 0;
    let mut atcoder = HashMap::new();
    let atcoder_vec = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for i in 0..atcoder_vec.len() {
        atcoder.insert(atcoder_vec[i], 0);
    }

    for i in 0..lowercase.len() {
        if atcoder.contains_key(&lowercase[i]) {
            diff += (moji_num_s[i] - moji_num_t[i]).abs();
        }
        else {
            if moji_num_s[i] != moji_num_t[i] {
                println!("No");
                return;
            }
        }
    }

    if diff > at_num_s + at_num_t {
        println!("No");
    }
    else {
        println!("Yes");
    }


}