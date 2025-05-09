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
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }

    let xys = [(2,1),(1,2),(-1,2),(-2,1),(-2,-1),(-1,-2),(1,-2),(2,-1)];

    for (dx,dy) in xys {
        let xc = x1 + dx;
        let yc = y1 + dy;
        if (x2 - xc) * (x2 - xc) + (y2 - yc) * (y2 - yc) == 5 {
            println!("Yes");
            return;
        }

    }
    println!("No");

}