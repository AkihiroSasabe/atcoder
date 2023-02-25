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
        q: usize,
    }
    let mut kind = vec![];
    let mut xx = vec![];
    for i in 0..q {
        input! {
            kind_i: usize,
            x_i: usize
        }
        kind.push(kind_i);
        xx.push(x_i);
    }
    let mut count = vec![0; n+1];
    for i in 0..q {
        if kind[i] == 1 {
            count[xx[i]] += 1;
        }
        else if kind[i] == 2 {
            count[xx[i]] += 2;
        }
        else if kind[i] == 3 {
            if count[xx[i]] >= 2 {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
    }


}