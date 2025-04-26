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
    // 2025-04-25
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    // (y,x)にマスがある -> (y,x), (y+1,x),(y,x+1),(y+1,x+1)に格子点がある。
    let mut latice = vec![vec![false; w+5]; h+5];

    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                latice[y][x] = true;
                latice[y+1][x] = true;
                latice[y][x+1] = true;
                latice[y+1][x+1] = true;
            }
        }
    }

    let mut ans = 0;
    for y in 0..h+1 {
        for x in 0..w+1 {
            if !latice[y][x] {
                continue;
            }
            // カーネルチェック
            // 左上が凸な角
            if s[y][x] == '#' && s[y-1][x] == '.' && s[y][x-1] == '.' && s[y-1][x-1] == '.' {
                // println!("a, (y, x) = {:?}", (y, x));
                ans += 1;
            }
            // 左上が凹な角
            else if s[y][x] == '#' && s[y-1][x] == '#' && s[y][x-1] == '#' && s[y-1][x-1] == '.' {
                // println!("b, (y, x) = {:?}", (y, x));
                ans += 1;
            }

            // 左下が凸な角
            if s[y][x] == '.' && s[y-1][x] == '#' && s[y][x-1] == '.' && s[y-1][x-1] == '.' {
                // println!("a2, (y, x) = {:?}", (y, x));
                ans += 1;
            }
            // 左下が凹な角
            else if s[y][x] == '#' && s[y-1][x] == '#' && s[y][x-1] == '.' && s[y-1][x-1] == '#' {
                // println!("b2, (y, x) = {:?}", (y, x));
                ans += 1;
            }

            // 右上が凸な角
            else if s[y][x] == '.' && s[y-1][x] == '.' && s[y][x-1] == '.' && s[y-1][x-1] == '#' {
                // println!("c, (y, x) = {:?}", (y, x));
                ans += 1;
            }
            // 右上が凹な角
            else if s[y][x] == '.' && s[y-1][x] == '#' && s[y][x-1] == '#' && s[y-1][x-1] == '#' {
                // println!("d, (y, x) = {:?}", (y, x));
                ans += 1;
            }

            // 右上が凸な角
            else if s[y][x] == '.' && s[y-1][x] == '.' && s[y][x-1] == '#' && s[y-1][x-1] == '.' {
                // println!("c2, (y, x) = {:?}", (y, x));
                ans += 1;
            }
            // 右上が凹な角
            else if s[y][x] == '#' && s[y-1][x] == '.' && s[y][x-1] == '#' && s[y-1][x-1] == '#' {
                // println!("d2, (y, x) = {:?}", (y, x));
                ans += 1;
            }
        }
    }
    println!("{}", ans);



}