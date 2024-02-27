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
        p: [usize; n],
        q: usize,
    }

    let mut orders = vec![0; n];
    for i in 0..n {
        orders[p[i]-1] = i;
    }

    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input! {
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    for i in 0..q {
        let ai = a[i];
        let bi = b[i];
        if orders[ai - 1] < orders[bi - 1] {
            println!("{}", ai);
        }
        else {
            println!("{}", bi);
        }
    }

}