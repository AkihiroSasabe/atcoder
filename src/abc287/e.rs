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
        mut s: [String; n]
    }

    // let aaa = s[0].chars().nth(0).unwrap();
    // println!("{:?}", aaa);

    let mut ss = vec![];
    for i in 0..n {
        println!("{}", i);
        let i_string: String = i.to_string();
        ss.push(vec![s[i].clone(), i_string]);
        // println!("{:?}", s[i]);
    }
    ss.sort();
    let mut post = 0;
    for j in 0..min(ss[0][0].len(), ss[1][0].len()) {
        let c0 = s[0][0].chars().nth(0).unwrap();
        let c1 = s[1][0].chars().nth(0).unwrap();

        if c0 != c1 {
            post = j;
            break
        }
    }
    for i in 0..n-1 {
        for j in 0..ss[i][0].len() {
            
        }
        println!("{:?}", ss[i]);
    }
}