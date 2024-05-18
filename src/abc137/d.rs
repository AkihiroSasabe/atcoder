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
    // 2024-05-18 12:32-12:55 (23min)
    input! {
        n: usize, // N件のバイト
        m: usize, // m日あとまでに、稼げる最大
    }
    let mut ab = vec![];
    for i in 0..n {
        input!{
            ai: usize, // ai日あと
            bi: usize,  // 報酬
        }
        ab.push((ai, bi));
    }
    ab.sort();
    ab.reverse();

    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for i in (0..m).rev() {
        while let Some((aj, bj)) = ab.pop() {
            // println!("aj = {aj}, i = {i}, aj + i = {}", aj + i);
            if aj + i <= m {
                heap.push(bj);
            }
            else {
                ab.push((aj, bj));
                break;
            }
        }
        // println!("i = {:?}", i);
        // println!("heap = {:?}", heap);
        if let Some(v) = heap.pop() {
            ans  += v;
        }
    }
    println!("{}", ans);
}