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
        s: Chars        
    }
    let mut ans = 0;
    for i in 0..s.len() {
        if s[i] == 'v' {
            ans += 1;
        }
        else {
            ans += 2;
        }
    }
    println!("{}", ans);


}