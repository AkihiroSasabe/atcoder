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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut items = HashMap::new();
    for i in 0..m {
        input! {
            x_i: isize,
            y_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
        items.insert(vec![x_i, y_i], 0);
    }

    let mut tx: isize = 0;
    let mut ty: isize = 0;
    
    for i in 0..n {
        // println!("(tx, tx, h) = ({}, {}, {})", tx, ty, h);
        if s[i] == 'R' {
            tx += 1;
        }
        else if s[i] == 'L' {
            tx -= 1;
        }
        else if s[i] == 'U' {
            ty += 1;
        }
        else if s[i] == 'D' {
            ty -= 1;
        }
        h -= 1;
        if h < 0 {
            println!("No");
            return
        }
        if items.contains_key(&vec![tx, ty]) {
            if h < k {
                h = k;
                items.remove(&vec![tx, ty]);
            }
        }
    }
    println!("Yes");

}