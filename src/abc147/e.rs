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
use std::vec;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2025-08-02 14:13-11:05-11:40 (35min)
    input! {
        h: usize,
        w: usize,
        mut a: [[isize; w]; h],
        mut b: [[isize; w]; h],
    }
    for i in 0..h {
        a[i].push(0);
        b[i].push(0);
    }
    a.push(vec![0; w+1]);
    b.push(vec![0; w+1]);

    // 12_800
    // 81_920_000

    // dp[y][x][diff+80*(h+w)] := diffが可能か?
    let mut dp = vec![vec![vec![false; 160*(h+w)+2]; w+1]; h+1];

    let offset = 80 * (h + w) as isize;
    dp[0][0][(offset + a[0][0] - b[0][0]) as usize] = true;
    dp[0][0][(offset - a[0][0] + b[0][0]) as usize] = true;

    for y in 0..h {
        for x in 0..w {
            // println!("dp[{}][{}] = {:?}", y, x, dp[y][x]);
            for diff in 0..(160*(h+w) as isize +1) {
                // 右に進む
                let d0 = a[y][x+1] - b[y][x+1];
                let d1 = - a[y][x+1] + b[y][x+1];
                let ndiff0 = diff + d0;
                let ndiff1 = diff + d1;

                if dp[y][x][diff as usize] {
                    if ndiff0 >= 0 && ndiff0 <= 160*(h+w) as isize + 1 {
                        dp[y][x+1][ndiff0 as usize] = true;
                    }
                    if ndiff1 >= 0 && ndiff1 <= 160*(h+w) as isize + 1 {
                        dp[y][x+1][ndiff1 as usize] = true;
                    }
                }

                // 下に進む
                let d0 = a[y+1][x] - b[y+1][x];
                let d1 = - a[y+1][x] + b[y+1][x];
                let ndiff0 = diff + d0;
                let ndiff1 = diff + d1;

                if dp[y][x][diff as usize] {
                    if ndiff0 >= 0 && ndiff0 <= 160*(h+w) as isize + 1 {
                        dp[y+1][x][ndiff0 as usize] = true;
                    }
                    if ndiff1 >= 0 && ndiff1 <= 160*(h+w) as isize + 1 {
                        dp[y+1][x][ndiff1 as usize] = true;
                    }
                }
            }
        }
    }

    let mut ans = 80 * (h + w) as isize;
    for diff in 0..(160*(h+w) as isize + 1) {
        if dp[h-1][w-1][diff as usize] {
            ans = min(ans, (diff - offset).abs());
        }
    }
    println!("{}", ans);
}