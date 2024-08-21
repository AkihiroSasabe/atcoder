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
        mut x: Chars
    }
    // x.reverse();
    // let n = x.len();
    // if x[n-1] == '0' {
    //     x.pop();
    // }

    while x[x.len()-1] == '0' {
        x.pop();
    }
    if x[x.len()-1] == '.' {
        x.pop();
    }
    for i in 0..x.len() {
        print!("{}", x[i]);
    }



}