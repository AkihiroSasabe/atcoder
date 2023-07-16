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
        s: [Chars; n]
    }

    let mut s_rev = vec![];
    for i in 0..n {
        let mut aaa = s[i].clone();
        aaa.reverse();
        s_rev.push(aaa);
    }
    // println!("s={:?}", s);
    // println!("s_rev={:?}", s_rev);

    // 対照なやつは除く
    let mut hash = HashMap::new();
    let mut sym_hash = HashMap::new();
    // let mut sym = 0;
    for i in 0..n {
        if s_rev[i] == s[i] {
            // sym += 1;
            sym_hash.insert(s[i].clone(), 0);
        }
        else {
            if !hash.contains_key(&s[i]) {
                if !hash.contains_key(&s_rev[i]) {
                    hash.insert(s[i].clone(), 0);
                    hash.insert(s_rev[i].clone(), 0);
                }
            }
        }
    }

    let mut ans = sym_hash.len() + (hash.len() / 2);
    println!("{}", ans);



}