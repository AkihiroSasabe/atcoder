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
        d: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for y0 in 0..h {
        for x0 in 0..w {
            for y1 in 0..h {
                for x1 in 0..w {
                    let mut temp: Vec<Vec<usize>> = vec![vec![0; w]; h];
                    if s[y0][x0] == '.' && s[y1][x1] == '.' {
                        for dy in 0..(d+1) {
                            for dx in 0..(d-dy+1) {
                                for ys in [-1, 1] {
                                    for xs in [-1, 1] {
                                        let ny0 = y0 as isize + dy as isize * ys;
                                        let nx0 = x0 as isize + dx as isize * xs;
                                        if !(ny0 < 0 || h as isize <= ny0 || nx0 < 0 || w as isize <= nx0) {
                                            let ny0 = ny0 as usize;
                                            let nx0 = nx0 as usize;
                                            temp[ny0][nx0] = 1;
                                        }
                                        
                                        let ny1 = y1 as isize + dy as isize * ys;
                                        let nx1 = x1 as isize + dx as isize * xs;
                                        if !(ny1 < 0 || h as isize <= ny1 || nx1 < 0 || w as isize <= nx1) {
                                            let ny1 = ny1 as usize;
                                            let nx1 = nx1 as usize;
                                            temp[ny1][nx1] = 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let mut cand = 0;
                    for yy in 0..h {
                        for xx in 0..w {
                            if s[yy][xx] == '#' {continue}
                            cand += temp[yy][xx];
                        }
                    }
                    ans = max(ans, cand);
                }
            }
        }
    }
    println!("{}", ans);

}