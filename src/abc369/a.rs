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
        a: isize,
        b: isize,
    }
    let big = max(a, b);
    let sma = min(a, b);

    if a == b {
        println!("1");
        return
    }

    if (big - sma) % 2 == 0 {
        println!("3");
        return
    }

    println!("2");
    

}