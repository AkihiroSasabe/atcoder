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
        n: usize,
        a: [usize; n]
    }
    let mut axy = vec![vec![]; n];
    let mut flag = true;
    let INF = 1 << 60;
    let mut min_exp2 = INF;
    let mut min_exp3 = INF;
    for i in 0..n {
        axy[i] = get_axy(a[i]);
        min_exp2 = min(min_exp2, axy[i][1]);
        min_exp3 = min(min_exp3, axy[i][2]);
        if axy[i][0] != axy[0][0] {
            flag = false;
        }
        // println!("{:?}", axy[i]);
    }
    let mut ans = 0;
    if flag {
        for i in 0..n {
            ans = ans + axy[i][1] - min_exp2;
            ans = ans + axy[i][2] - min_exp3;
        }
        println!("{}", ans);
    }
    else {
        println!("-1");
    }

}

fn get_axy(mut x: usize) -> Vec<usize> {
    let mut exp2 = 0;
    let mut exp3 = 0;
    while x % 2 == 0 {
        x /= 2;
        exp2 += 1;
    }
    while x % 3 == 0 {
        x /= 3;
        exp3 += 1;
    }
    return vec![x, exp2, exp3]
}