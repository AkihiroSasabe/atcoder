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
use rand::Rng;
fn main() {
    input! {
        mut m: usize
    }

    let mut a = vec![];
    let mut pow3 = 1;
    for i in 0..11 {
        let r = m % 3;
        for _ in 0..r {
            a.push(i);
        }
        m /= 3;
    }
    println!("{}", a.len());
    for ai in a {
        print!("{} ", ai);
    }
    
}