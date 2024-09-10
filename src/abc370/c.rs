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
        mut s: Chars,
        t: Chars,
    }

    let mut firsts = vec![];
    let mut laters = vec![];
    for i in 0..s.len() {
        if s[i] > t[i] {
            firsts.push(i);
        }
        else if s[i] < t[i] {
            laters.push(i);
        }
    }
    laters.reverse();

    let mut x = vec![];
    for i in firsts {
        s[i] = t[i];
        x.push(s.clone());
    }
    for i in laters {
        s[i] = t[i];
        x.push(s.clone());
    }

    // x.sort();
    println!("{}", x.len());
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            print!("{}", x[i][j]);
        }
        println!("");
    }



}