#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
        n: usize,
        mut r: isize
    }
    let mut d = vec![];
    let mut a = vec![];
    for i in 0..n {
        input!{
            di: usize,
            ai: isize,
        }
        d.push(di);
        a.push(ai);
    }

    for i in 0..n {
        if d[i] == 1 {
            if !(1600 <= r && r <= 2799) {
                continue
            }
        }
        else if d[i] == 2 {
            if !(1200 <= r && r <= 2399) {
                continue
            }
        }

        r = r + a[i];
    }
    println!("{}", r);
}