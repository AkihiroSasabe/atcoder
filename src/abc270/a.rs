use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        a: isize,
        b: isize,
    }
    
    let mut takahshi = vec![false; 3];
    let mut aoki = vec![false; 3];
    // let mut sunuke = vec![false; 3];

    for i in 0..3 {
        if a & (1 << i) != 0 {
            takahshi[i] = true;
        }
    }
    for i in 0..3 {
        if b & (1 << i) != 0 {
            aoki[i] = true;
        }
    }

    let mut ans = 0;
    for i in 0..3 {
        if takahshi[i] || aoki[i] {
            ans += (1 << i);
        }
    }
    println!("{}", ans);


}