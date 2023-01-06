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
    // 2023-01-06 12:53-
    // 13:10 (NG)       問題文の通りに実装した(TLE)
    // 13:23 (AC)       BTreeMapを使った
    // 30min
    input! {
        q: usize
    }
    let mut t = vec![];
    let mut x = vec![];
    for i in 0..q {
        input! {
            t_i: usize,
            x_i: usize,
        }
        t.push(t_i);
        x.push(x_i);
    }
    let N = 2_usize.pow(20 as u32);
    let mut btree = BTreeMap::new();
    for i in 0..N {
        btree.insert(i, 1);
    }

    // 2^20 = 1_048_576 = 10^6
    let mut a = vec![-1; N];
    for i in 0..q {
        if t[i] == 1 {
            let mut h = x[i];
            if a[h % N] != -1 {
                if let Some((&key, _)) = btree.range((h % N)..).next() {
                    h = key;
                    // println!("{}", h);
                }
                else {
                    let (&key, _) = btree.range(0..).next().unwrap();
                    h = key;
                }
            }
            // while a[h % N] != -1 {
            //     h += 1;
            // }
            a[h%N] = x[i] as isize;
            btree.remove(&(h%N));
        }
        else if t[i] == 2 {
            println!("{}", a[x[i] % N]); 
        }
    }
}