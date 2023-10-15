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
        s: [Chars; n]
    }
    for i in 0..n-8 {
        for j in 0..m-8 {
            if check(i, j, &s) {
                println!("{} {}", i+1, j+1);
            }
        }
    }


}

fn check(y: usize, x: usize, s: &Vec<Vec<char>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let yy = i + y;
            let xx = j + x;
            let yyy = 6 + i + y;
            let xxx = 6 + j + x;
            if s[yy][xx] != '#' {
                return false
            }
            if s[yyy][xxx] != '#' {
                return false
            }
        }
    }
    for i in 0..4 {
        let yy = y + i;
        let xx = x + 3;
        if s[yy][xx] == '#' {
            return false
        }
        let yy = y + 3;
        let xx = x + i;
        if s[yy][xx] == '#' {
            return false
        }

        let yy = y + 5 + i;
        let xx = x + 5;
        if s[yy][xx] == '#' {
            return false
        }
        let yy = y + 5;
        let xx = x + 5 + i;
        if s[yy][xx] == '#' {
            return false
        }
    }
    





    return true
}