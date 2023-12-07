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
    // 2023-12-07 17:07-17:30 (23min)
    // 2023-12-07 20:44-21:18 (34min)
    // total 57min
    input! {
        n: usize,
        x: isize,
        y: isize,
        a: [isize; n],
    }

    let mut ans = 0;
    let mut x_inds = vec![]; // xのあるindex
    let mut y_inds = vec![]; // yのあるindex
    let mut bans = vec![]; // y未満かxを超えるindex

    for i in 0..n {
        if a[i] == x {
            x_inds.push(i);
        }
        if a[i] == y {
            y_inds.push(i);
        }
        if a[i] < y || x < a[i] {
            bans.push(i);
        }
    }
    bans.push(n);
    let mut b_ind = 0;
    let mut next_ban = bans[0];

    // println!("bans = {:?}", bans);
    // println!("x_inds = {:?}", x_inds);
    // println!("y_inds = {:?}", y_inds);

    // ban[j]とban[j+1]の領域で許されるものを数える
    for i in 0..n {
        if i == next_ban {
            b_ind += 1;
            if b_ind < bans.len() {
                next_ban = bans[b_ind];
            }
            continue
        }
        // 現在値iから、次のバン位置までで許される領域の長さを求める
        let x_lb = x_inds.lower_bound(&i);
        let y_lb = y_inds.lower_bound(&i);
        if x_lb == x_inds.len() {continue}
        if y_lb == y_inds.len() {continue}

        let x_ind = x_inds[x_lb];
        let y_ind = y_inds[y_lb];

        if x_ind >= next_ban {continue}
        if y_ind >= next_ban {continue}

        let max_ind = max(x_ind, y_ind);
        ans += next_ban - max_ind
    }

    println!("{}", ans);

}
