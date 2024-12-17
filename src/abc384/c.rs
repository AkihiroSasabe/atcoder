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
        // a: isize,
        // b: isize,
        // c: isize,
        // d: isize,
        // e: isize,
        ps: [isize; 5]
    }

    let cs = vec!['A', 'B', 'C', 'D', 'E'];

    let mut hitos = vec![];
    for mask in 1..1<<5 {
        let mut hito = vec![];
        let mut p = 0;
        for i in 0..5 {
            if mask & (1 << i) != 0 {
                hito.push(cs[i]);
                p += ps[i];
            }
        }
        hitos.push((-p, hito));
    }
    hitos.sort();
    for (p, hito) in hitos {
        for h in hito {
            print!("{}", h);
        }
        println!("");
    }


}