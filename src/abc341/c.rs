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
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        s: [Chars; h]
    }

    let mut t2: Vec<usize> = vec![];
    for i in 0..n {
        let ti = match t[i] {
            'R' => 0,
            'U' => 1,
            'L' => 2,
            'D' => 3,
            _ => 4
        };
        t2.push(ti);
    }
    // println!("t2 = {:?}", t2);

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    let mut ans = 0;

    for mut ys in 0..h {
        for mut xs in 0..w {
            // println!("---- (ys,xs) = {} {} ----", ys, xs);
            let mut flag = true;
            if s[ys][xs] == '#' {continue}
            let mut x = xs;
            let mut y = ys;
            for i in 0..n {
                let d = t2[i];
                let next_y = (dir_y[d] + y as isize) as usize;
                let next_x = (dir_x[d] + x as isize) as usize;
                // println!("(next_y, next_x) = {} {}", next_y, next_x);
                if s[next_y][next_x] == '#' {
                    flag = false;
                    break
                }
                y = next_y;
                x = next_x;
            }
            if flag {
                ans += 1;
            }
        }
    }
    println!("{}", ans);





}