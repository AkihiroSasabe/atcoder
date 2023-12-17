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
        k: usize,
        g: usize,
        m: usize,
    }

    let mut gc = 0;
    let mut mc = 0;
    for i in 0..k {
        if gc == g {
            gc = 0;
        }
        else if mc == 0 {
            mc = m;
        }
        else {
            let diff = g - gc;
            if diff > mc {
                gc += mc;
                mc = 0;
            }
            else {
                gc = g;
                mc -= diff;
            }
        }
    }
    println!("{} {}", gc, mc);
}