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
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    solve();
}

fn solve() {
    // evimaさんのやり方を、解説AC
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    let INF = 1_000_000_000;
    let a = a + INF;
    let b = b + INF;
    let c = c + INF;
    let d = d + INF;

    let mut s = vec![vec![0; 4]; 2];
    s[0][0] = 2;
    s[0][1] = 1;
    s[0][2] = 0;
    s[0][3] = 1;

    s[1][0] = 1;
    s[1][1] = 2;
    s[1][2] = 1;
    s[1][3] = 0;

    let mut cum = vec![vec![0; 4]; 2];
    cum[0][0] = s[0][0];
    cum[1][0] = s[1][0];
    for x in 1..4 {
        cum[0][x] = cum[0][x-1] + s[0][x];
        cum[1][x] = cum[1][x-1] + s[1][x];
    }
    for x in 0..4 {
        cum[1][x] += cum[0][x];
    }

    // println!("cum = {:?}", cum);

    let s0 = get_area_from_origin(c, d, &cum);
    let s1 = get_area_from_origin(a, b, &cum);
    let s2 = get_area_from_origin(a, d, &cum);
    let s3 = get_area_from_origin(c, b, &cum);

    // println!("s0 = {:?}", s0);
    // println!("s1 = {:?}", s1);
    // println!("s2 = {:?}", s2);
    // println!("s3 = {:?}", s3);

    let ans = s0 + s1 - s2 - s3;
    println!("{}", ans);


}

fn get_area_from_origin(x: isize, y: isize, cum: &Vec<Vec<isize>>) -> isize {
    let rx = (x % 4) as usize;
    let ry = (y % 2) as usize;

    let tx = (x / 4) * 4;
    let ty = (y / 2) * 2;

    let s0 = (tx / 4) * (ty / 2) * cum[1][3];

    let s1 = if ry != 0 {
         (tx / 4) * cum[ry - 1][3]
    }
    else {
        0
    };

    let s2 = if rx != 0 {
        (ty / 2) * cum[1][rx-1]
    }
    else {
        0
    };

    let s3 = if ry != 0 && rx != 0 {
        cum[ry-1][rx-1]
    }
    else {
        0
    };

    let ans = s0 + s1 + s2 + s3;
    return ans
}

fn solve_honban() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    let INF = 1_000_000_000_0;
    let a = a + INF;
    let b = b + INF;
    let c = c + INF;
    let d = d + INF;

    // let rx = a % 2;
    // let ry = b % 4;

    let rbx = a % 4;
    let rby = b % 4;
    let rtx = c % 4;
    let rty = d % 4;

    let mut bx = (a / 4) * 4;
    let mut by = (b / 4) * 4;
    if rbx != 0 {
        bx += 4;
    }
    if rby != 0 {
        by += 4;
    }

    let tx = (c / 4) * 4;
    let ty = (d / 4) * 4;


    let mut ans = 0;
    ans = (tx - bx) * (ty - by) / 8 * 6 * 2;
    let mut aaa = vec![vec![0; 4]; 4];
    aaa[0][0] = 2;
    aaa[0][1] = 3;
    aaa[0][2] = 5;
    aaa[0][3] = 6;

    aaa[1][0] = 3;
    aaa[1][1] = 6;
    aaa[1][2] = 7;
    aaa[1][3] = 8;

    aaa[2][0] = 5;
    aaa[2][1] = 7;
    aaa[2][2] = 10;
    aaa[2][3] = 12;

    aaa[3][0] = 6;
    aaa[3][1] = 12;
    aaa[3][2] = 14;
    aaa[3][3] = 16;


    let mut s0 = 0;
    let mut s1 = 0;
    let mut s2 = 0;

    let mut s3 = 0;
    let mut s4 = 0;
    
    let mut s5 = 0;
    let mut s6 = 0;
    let mut s7 = 0;

    if rbx != 0 && rby != 0 {
        s0 = 12 + aaa[rby as usize - 1][rbx as usize - 1] - aaa[rby as usize - 1][3] - aaa[3][rbx as usize - 1];
    }
    if rby != 0 {
        s1 = (tx - bx) / 4 * (12 - aaa[rby as usize-1][3]);
    }
    if rtx != 0 && rby != 0 {
        s2 = aaa[3][rtx as usize - 1] - aaa[rby as usize - 1][rtx as usize - 1];
    }
    if rbx != 0 {
        s3 = (ty - by) / 4 * (12 - aaa[3][rbx as usize - 1]);
    }
    if rtx != 0 {
        s4 = (ty - by) / 4 * aaa[3][rtx as usize - 1];
    }

    ans += s0 + s1 + s2 + s3 + s4 + s5 + s6 + s7;
}