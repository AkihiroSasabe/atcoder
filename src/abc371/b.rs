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
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..m {
        input! {
            ai: usize, // 番号
            bi: char, // M or F
        }
        a.push(ai-1);
        b.push(bi);
    }

    let mut is_first = vec![true; n];
    for i in 0..m {
        let ai = a[i];
        if is_first[ai] && b[i] == 'M' {
            println!("Yes");
        }
        else {
            println!("No");
        }
        if b[i] == 'M' {
            is_first[ai] = false;
        }


    }




}