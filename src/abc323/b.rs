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
        s: [Chars; n]
    }

    let mut katis = vec![];
    for i in 0..n {
        let mut kati: isize = 0;
        for j in 0..s[i].len() {
            if s[i][j] == 'o' {
                kati += 1;
            }
        }
        katis.push(vec![- kati, 1 + i as isize]);
    }
    katis.sort();
    for i in 0..n {
        print!("{} ", katis[i][1]);
    }


}