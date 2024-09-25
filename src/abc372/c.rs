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
        n: usize,
        q: usize,
        mut s: Chars,
    }
    let mut x = vec![];
    let mut c = vec![];
    for i in 0..q {
        input!{
            xi: usize,
            ci: char,
        }
        x.push(xi-1);
        c.push(ci);
    }
    let abc = vec!['A', 'B', 'C'];
    let mut set = BTreeSet::new();
    for i in 0..n-2 {
        if s[i..i+3] == abc {
            set.insert(i);
        }
    }


    for i in 0..q {
        let xi = x[i];
        for j in 0..3 {
            if xi < j {continue}
            let ind = xi - j;
            set.remove(&ind);
        }
        s[xi] = c[i];
        for j in 0..3 {
            if xi < j {continue}
            let ind = xi - j;
            if xi - j + 3 > n {continue}
            if s[xi-j..xi-j+3] == abc {
                set.insert(xi-j);
            }
        }
        println!("{}", set.len());
    }
}