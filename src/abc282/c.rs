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
        mut s: Chars
    }

    let mut index = vec![];
    for i in 0..n {
        if s[i] == ',' {
            index.push(i);
        }
    }

    let mut ss = 0;
    let mut tt = 0;
    // let mut change_index = vec![];
    let mut change_index = HashMap::new();
    loop {
        if s[ss] == '"' {
            tt = ss + 1;
            while s[tt] != '"' {
                if s[tt] == ',' {
                    // change_index.push(tt);
                    change_index.insert(tt, 1);
                }
                tt += 1;
                if tt >= n {break}
            }
            ss = tt;
        }
        ss += 1;
        if ss >= n || tt >= n {break}
    }

    // println!("{:?}", change_index);

    for i in 0..index.len() {
        if !change_index.contains_key(&index[i]) {
            s[index[i]] = '.';            
        }
    }
    for i in 0..n {
        print!("{}", s[i]);
    }

}