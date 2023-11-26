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
        l: usize,
        r: usize,
        a: [usize; n],
    }

    for i in 0..n {
        if a[i] < l {
            print!("{} ", l);
        }
        else if r < a[i] {
            print!("{} ", r);
        }
        else {
            print!("{} ", a[i]);
        }

    }

}