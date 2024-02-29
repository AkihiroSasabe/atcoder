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
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut s = vec![];
    let mut t = vec![];
    for i in 0..n-1 {
        input!{
            si: usize,
            ti: usize,
        }
        s.push(si);
        t.push(ti)
    }
    for i in 0..n-1 {
        let num = a[i] / s[i];
        a[i+1] += num * t[i]; 
    }

    println!("{}", a[n-1]);
    

}