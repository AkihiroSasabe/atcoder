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
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }
    let mut hash = HashSet::new();
    // for i in 0..q {
    //     hash.insert(x[i]);
    // }
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                let sum = a[i] + b[j] + c[k];
                hash.insert(sum);
                // if hash.contains(&sum) {
                    // println!("Yes");
                    // return
                // }
            }
        }
    }
    for i in 0..q {
        if hash.contains(&x[i]) {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }


}