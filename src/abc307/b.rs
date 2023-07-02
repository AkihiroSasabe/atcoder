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
    let mut flag = false;

    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            let mut gattai = s[i].clone();
            for k in 0..s[j].len() {
                gattai.push(s[j][k]);
            }
            let mut flag2 = true;
            for k in 0..gattai.len() {
                if gattai[k] != gattai[gattai.len()-1-k] {
                    flag2 = false;
                }
            }
            if flag2 {
                flag = true;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}