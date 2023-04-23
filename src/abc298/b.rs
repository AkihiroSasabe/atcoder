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
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    for i in 0..4 {
        let mut flag = check(&a, &b, n);
        if flag {
            println!("Yes");
            return
        }
        a = rotate(a, n);
    }
    println!("No");

}

fn check(a: & Vec<Vec<usize>>, b: & Vec<Vec<usize>>, n: usize) -> bool {
    for y in 0..n {
        for x in 0..n {
            if a[y][x] == 1 {
                if b[y][x] != 1 {
                    return false;
                }
            }
        }
    }
    return true
}

fn rotate(a: Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut new_a = vec![vec![0; n]; n];
    for y in 0..n {
        for x in 0..n {
            new_a[y][x] = a[n-1-x][y];
        }
    }
    return new_a
}