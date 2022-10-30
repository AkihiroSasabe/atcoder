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
        n: usize
    }
    // let mut ff = vec![0; n+1];
    // ff[0] = 1;
    let mut ff: HashMap<usize, usize> = HashMap::new();
    ff.insert(0, 1);
    let ans = get_f(n, &mut ff);
    println!("{}", ans);
}

fn get_f(x: usize, ff: &mut HashMap<usize, usize>) -> usize {
    // println!("x: {}, ff[x]: {}", x, ff[x]);
    if !ff.contains_key(&x) {
        let val = get_f(x/2, ff) + get_f(x/3, ff);
        ff.insert(x, val);
    }
    // if ff[x] == 0 {
    //     ff[x] = get_f(x/2, ff) + get_f(x/3, ff);
    // }
    return ff[&x]
}