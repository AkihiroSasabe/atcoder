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
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-14 (Sat.) 
    // 2023-10-15 (Sun.) 9:19-9:26 (7min)
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n]
    }

    let mut ans = vec![];
    for i in 0..n {
        // println!("i = {i}, ans = {:?}", ans);
        if t == s[i] {
            ans.push(i+1);
        }
        else if s[i].len() == t.len() + 1 {
            let mut count = 0;
            let mut s_ind = 0;
            let mut t_ind = 0;

            loop {
                if s[i][s_ind] != t[t_ind] {
                    count += 1;
                    s_ind += 1;
                }
                else {
                    t_ind += 1;
                    s_ind += 1;
                }
                if count >= 2 {break}
                if s_ind >= s[i].len() || t_ind >= t.len() {break}
            }
            if count <= 1 {
                ans.push(i + 1);
            }
        }
        else if s[i].len() == t.len() - 1 {
            let mut count = 0;
            let mut s_ind = 0;
            let mut t_ind = 0;

            loop {
                if s[i][s_ind] != t[t_ind] {
                    count += 1;
                    t_ind += 1;
                }
                else {
                    t_ind += 1;
                    s_ind += 1;
                }
                if count >= 2 {break}
                if s_ind >= s[i].len() || t_ind >= t.len() {break}
            }
            if count <= 1 {
                ans.push(i + 1);
            }
        }
        else if t.len() == s[i].len() {
            let mut count = 0;
            for j in 0..t.len() {
                if t[j] != s[i][j] {
                    count += 1;
                    if count >= 2 {
                        break;
                    }
                }
            }
            if count <= 1 {
                ans.push(i + 1);
            }
        }
        
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }


}