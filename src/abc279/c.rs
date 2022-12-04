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
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    // let mut a = vec!['a', 'b'];
    // let mut b = vec!['a', 'c'];
    // if a == b {
    //     println!("ok");
    // }
    // else {
    //     println!("no");
    // }


    let mut s_t = vec![];
    let mut t_t = vec![];
    for i in 0..w {
        let mut s_temp = vec![];
        let mut t_temp = vec![];
        for j in 0..h {
            s_temp.push(s[j][i]);
            t_temp.push(t[j][i]);
        }
        s_t.push(s_temp);
        t_t.push(t_temp);
    }

    let mut s_hash_map = HashMap::new();
    let mut t_hash_map = HashMap::new();
    for i in 0..w {
        if !s_hash_map.contains_key(&s_t[i]) {
            s_hash_map.insert(&s_t[i], 1);
        }
        else {
            *s_hash_map.get_mut(&s_t[i]).unwrap() += 1;
        }
        if !t_hash_map.contains_key(&t_t[i]) {
            t_hash_map.insert(&t_t[i], 1);
        }
        else {
            *t_hash_map.get_mut(&t_t[i]).unwrap() += 1;
        }
    }
    if s_hash_map == t_hash_map {
        println!("Yes");
    }
    else {
        println!("No");
    }



}