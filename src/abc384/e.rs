#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
fn main() {
    input! {
        h: usize,
        w: usize,
        xx: isize,
        p: Usize1, // 高橋 y
        q: Usize1, // たかはsい x
        s: [[isize; w]; h] // 強さ
    }

    let mut heap: BinaryHeap<(isize, usize, usize)> = BinaryHeap::new();
    let mut ans = 0;

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    let mut seen = vec![vec![false; w]; h];
    seen[p][q] = true;
    heap.push((-s[p][q], p, q));
    while heap.len() > 0 {
        let (mv, y, x) = heap.pop().unwrap();
        let v = - mv;
        if y == p && x == q {
            // 初回だけ特別対応
            ans += v;
        }
        else if (v as i128) * (xx as i128) < (ans as i128) {
            ans += v;
        } 
        else {
            break
        }
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if seen[ny][nx] {continue}
            heap.push((-s[ny][nx], ny, nx));
            seen[ny][nx] = true;
        }
    }
    println!("{}", ans);
}
