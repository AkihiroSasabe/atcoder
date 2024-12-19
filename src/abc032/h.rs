// https://atcoder.jp/contests/abc032/submissions/27198795

// -*- coding:utf-8-unix -*-
// #![feature(map_first_last)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::collections::*;
use std::convert::*;
use std::convert::{From, Into};
use std::error::Error;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::*;
use std::marker::Copy;
use std::mem::*;
use std::ops::Bound::*;
use std::ops::RangeBounds;
use std::ops::{Add, Mul, Neg, Sub};
use std::slice::from_raw_parts;
use std::str;
use std::vec;

const INF: i64 = 1223372036854775807;
const UINF: usize = INF as usize;
// const FINF: f64 = 122337203685.0;
const LINF: i64 = 2147483647;
const FINF: f64 = LINF as f64;
const INF128: i128 = 1223372036854775807000000000000;
const MOD: i64 = 1000000007;
// const MOD: i64 = 998244353;

const MPI: f64 = 3.14159265358979323846264338327950288f64;
// const MOD: i64 = INF;

const UMOD: usize = MOD as usize;
use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

macro_rules! p {
    ($x:expr) => {
        println!("{}", $x);
    };
}
macro_rules! d {
    ($x:expr) => {
        println!("{:?}", $x);
    };
}

// use str::Chars;

// use str::Chars;
#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn readi() -> (i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    iter.next().unwrap().parse::<i64>().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
#[allow(dead_code)]
fn read_mat<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

#[allow(dead_code)]
fn readii() -> (i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}

fn readff() -> (f64, f64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<f64>().unwrap(),
        iter.next().unwrap().parse::<f64>().unwrap(),
    )
}

#[allow(dead_code)]
fn readiii() -> (i64, i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}
#[allow(dead_code)]
fn readuu() -> (usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn readcc() -> (char, char) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<char>().unwrap(),
        iter.next().unwrap().parse::<char>().unwrap(),
    )
}

#[allow(dead_code)]
fn readuuu() -> (usize, usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

#[allow(dead_code)]
fn readuuuu() -> (usize, usize, usize, usize) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn readiiii() -> (i64, i64, i64, i64) {
    let mut str = String::new();
    let _ = stdin().read_line(&mut str).unwrap();
    let mut iter = str.split_whitespace();
    (
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
        iter.next().unwrap().parse::<i64>().unwrap(),
    )
}

pub struct Rheap {
    heap: BinaryHeap<Reverse<i64>>,
}

impl Rheap {
    // 0 <= size <= 10^8 is constrained.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn push(&mut self, x: i64) {
        self.heap.push(Reverse(x));
    }
    pub fn pop(&mut self, x: i64) -> Option<i64> {
        let x = self.heap.pop();
        if x == None {
            return None;
        }
        let Reverse(x) = x.unwrap();
        return Some(x);
    }
}
fn solve() {
    let (n, k) = readuu();
    /* -- two pointer --  */
    let mut vec = vec![0; n];
    let mut f = true;
    for i in 0..n {
        let x: usize = read();
        vec[i] = x;
        if x == 0 {
            f = false;
        }
    }
    if !f {
        println!("{:?}", n);
        return;
    }
    let mut r = 0 as usize;
    let mut acc = 1 as usize;
    let mut res = 0;
    for l in 0..n {
        while r < l {
            r += 1;
        }
        if (r < n) {
            while acc * vec[r] <= k && r < n {
                acc *= vec[r];
                r += 1;
                if r == n {
                    break;
                }
            }
        }

        res = max(res, r - l);
        // println!("{:?}", (l, r));
        if r != l {
            acc /= vec[l];
        }
    }

    println!("{:?}", res);

    return;
}

fn main() {
    solve();
}
