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
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: usize,
            yi: usize,
        }
        x.push(xi);
        y.push(yi);
    }
    input! {
        s: Chars
    }

    // 同じyで、x0 < x1 でRL なら衝突。
    let mut btree = BTreeMap::new();

    for i in 0..n {
        let xi = x[i];
        let yi = y[i];
        btree.entry(yi).or_insert(vec![]).push((xi, s[i]));
    }

    for (yi, mut vec) in btree {
        vec.sort();

        if vec.len() < 2 {continue}

        let mut is_right = false;
        let mut is_collision = false;
        // let mut is_left = false;
        for (xi, si) in vec {
            if is_right {
                if si == 'L' {
                    is_collision = true;
                }
            }
            if si == 'R' {
                is_right = true;
            }
        }
        if is_collision {
            println!("Yes");
            return;
        }
    }
    println!("No");


}