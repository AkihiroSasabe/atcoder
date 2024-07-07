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
        x0: usize,
        y0: usize,
        z0: usize,
        x1: usize,
        y1: usize,
        z1: usize,
        x2: usize,
        y2: usize,
        z2: usize,
        x3: usize,
        y3: usize,
        z3: usize,
    }
    // let mut p0 = (x0, y0, z0);
    // let mut p1 = (x1, y1, z1);
    // let mut p2 = (x2, y2, z2);
    // let mut p3 = (x3, y3, z3);



    // let mut is_ok = true;
    // if x0 < x2 {
    //     if x1 <= x2 {
    //         println!("No");
    //         return
    //     }
    // }
    // else {
    //     if x3 <= 
    // }

    if judge(x0, x1, x2, x3) && judge(y0, y1, y2, y3) && judge(z0, z1, z2, z3) {
        println!("Yes");
    }
    else {
        println!("No");
    }


}

fn judge(x0: usize, x1: usize, x2: usize, x3: usize) -> bool {
    if x1 <= x2 {
        return false
    }
    if x3 <= x0 {
        return false
    }
    return true
}