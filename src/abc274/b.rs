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
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    for i in 0..w {
        let mut x_i = 0;
        for j in 0..h {
            if c[j][i] == '#' {
                x_i += 1;
            }
        }
        print!("{} ", x_i);
    }

}