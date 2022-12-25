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
        h: usize,
        w: usize,
        mut a: [[usize; w]; h]
    }
    // for i in 0..h {
    //     println!("{:?}", a[i]);
    // }
    let mut ans = 0;
    let mut flag = true;
    let mut koritu = vec![];
    for i in 0..h {
        let koritu_flag = false;
        for j in 0..w {
            if is_koritu(&a, i, j) {
                koritu_flag = true;
                break
            }
        }
        if koritu_flag {
            for j in 0..w {
                if is_koritu(a[i][j], &a, i, j) {
            }
        }
    }


}

fn is_koritu(a: &Vec<Vec<usize>>, y:usize, x: usize) -> bool {
    let mut l = false;
    let mut r = false;
    let mut u = false;
    let mut d = false;
    let mut flag = false;

    if x > 0 {
        let (l_x, l_y) = (x-1, y);
        if v ==  a[l_y][l_x] {
            flag = true;
        }
    }
    if y > 0 {
        let (u_x, u_y) = (x, y-1);
        if v ==  a[u_y][u_x] {
            flag = true;
        }
    }
    let (r_x, r_y) = (x+1, y);
    if v ==  a[r_y][r_x] {
        flag = true;
    }
    let (d_x, d_y) = (x, y+1);
    if v ==  a[d_y][d_x] {
        flag = true;
    }
    return !flag

}