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
    input! {
        n: usize,
        a: [[[usize; n]; n]; n],
        q: usize,
        mut lx: [[usize; 6]; q]
    }

    // println!("a[0][1][2] = {:?}", a[0][1][2]);

    let mut lx2 = vec![vec![0; 6]; q];
    for i in 0..q {
        lx2[i][0] = lx[i][4];
        lx2[i][1] = lx[i][5];
        lx2[i][2] = lx[i][2];
        lx2[i][3] = lx[i][3];
        lx2[i][4] = lx[i][0];
        lx2[i][5] = lx[i][1];
    }

    solve(n, a, q, lx2);

}

fn solve(n: usize, a: Vec<Vec<Vec<usize>>>, q: usize, lx: Vec<Vec<usize>>) {
    // a[z][y][x] の順にアクセス
    let mut cum = vec![vec![vec![0; n]; n]; n];

    for z in 0..n {
        cum[z][0][0] = a[z][0][0];
        for x in 1..n {
            cum[z][0][x] = cum[z][0][x-1] + a[z][0][x];
        }
        for y in 1..n {
            cum[z][y][0] = cum[z][y-1][0] + a[z][y][0];
            for x in 1..n {
                cum[z][y][x] = cum[z][y][x-1] + a[z][y][x] + cum[z][y-1][x] - cum[z][y-1][x-1];
            }
        }
    }

    for z in 1..n {
        for y in 0..n {
            for x in 0..n {
                cum[z][y][x] += cum[z-1][y][x];
            }
        }
    }

    for i in 0..q {
        let x0 = lx[i][0] - 1;
        let x1 = lx[i][1] - 1;

        let y0 = lx[i][2] - 1;
        let y1 = lx[i][3] - 1;

        let z0 = lx[i][4] - 1;
        let z1 = lx[i][5] - 1;

        let mut b1 = 0;
        let mut b2 = 0;
        let mut b3 = 0;

        if y0 > 0 && x0 > 0 {
            b1 = cum[z1][y0-1][x0-1]; 
        }
        if x0 > 0 {
            b2 = cum[z1][y1][x0-1];
        }
        if y0 > 0 {
            b3 = cum[z1][y0-1][x1];
        }
        
        let mut ans = cum[z1][y1][x1] + b1 - b2 - b3;

        if z0 != 0 {
            let mut d1 = 0;
            let mut d2 = 0;
            let mut d3 = 0;
            if y0 > 0 && x0 > 0 {
                d1 = cum[z0-1][y0-1][x0-1]; 
            }
            if x0 > 0 {
                d2 = cum[z0-1][y1][x0-1];
            }
            if y0 > 0 {
                d3 = cum[z0-1][y0-1][x1];
            }
            let d = cum[z0-1][y1][x1] + d1 - d2 - d3;
            ans -= d;
        }
        println!("{}", ans);
    }
}