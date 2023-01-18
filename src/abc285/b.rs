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
        mut s: Chars
    }
    s.insert(0, 'x');
    for i in 1..n {
        let mut flag = false;
        for k in 1..(n+1-i) {
            // println!("s[{}]:{}, s[{}]:{}", k, s[k], i+k, s[i+k]);
            if s[k] == s[i+k] {
                println!("{}", k-1);
                flag = true;
                break
            }
        }
        if !flag {
            println!("{}", n-i);
        }
    }
}