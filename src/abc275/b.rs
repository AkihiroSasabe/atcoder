#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }
    let modulo = 998244353;
    let ans = ((((a % modulo) * (b % modulo)) % modulo * (c % modulo)) % modulo + modulo - (((d % modulo) * (e % modulo)) % modulo * (f % modulo)) % modulo ) % modulo;
    println!("{}", ans);



}