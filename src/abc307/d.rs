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
        s: Chars
    }
    let mut left = vec![];
    let mut right = BTreeMap::new();
    for i in 0..s.len() {
        if s[i] == '(' {
            left.push(i);
        }
        else if s[i] == ')' {
            right.insert(i, 0);
        }
    }

    let mut pairs = vec![];
    for i in 0..left.len() {
        let l_i = left[left.len()-1-i];

        let mut r_i = n;
        if let Some((&key, _)) = right.range(l_i+1..).next() {
            pairs.push(vec![l_i, key]);
            r_i = key;
            // println!("r_i= {}", r_i);
        }
        if r_i != n {
            right.remove(&r_i);
        }
    }
    pairs.sort();
    // println!("pairs={:?}", pairs);


    let mut left = n;
    let mut right = n;

    let mut pairs_index = 0;
    if pairs.len() != 0 {
        left = pairs[pairs_index][0];
        right = pairs[pairs_index][1];
        pairs_index += 1;
    }
    for i in 0..s.len() {
        if left <= i {
            if i < right {
                continue
            }
            if i == right {
                while left < i {
                    if pairs.len() != pairs_index {
                        left = pairs[pairs_index][0];
                        right = pairs[pairs_index][1];
                        pairs_index += 1;
                    }
                    else {
                        left = n;
                        right = n;
                    }                    
                }
                continue
            }
        }
        print!("{}", s[i]);
    }
}


// 追加のテストケース
// 25
// (aaa(bb)bb)ccc(dddd()eee)